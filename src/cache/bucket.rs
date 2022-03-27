use crate::cache::cl::{CacheDataEntry, CacheLine, ClCondidSlotsMask, ClSlot};
use crate::cache::cl_store::{ClIndex, ClStore, PerClStore};
// use std::intrinsics::size_of;
use std::ops::BitAnd;
// use core::mem::size_of;
// #[repr(C, align(64))]
pub struct Utility {
    pub first_set_bit: [i32; 256],
    pub last_set_bit: [i32; 256],
    pub num_set_bits: [u8; 256],
}

impl Utility {
    pub fn new() -> Utility {
        let mut u = Utility {
            first_set_bit: [-1; 256],
            last_set_bit: [-1; 256],
            num_set_bits: [0_u8; 256],
        };
        let set_bit: Vec<u8> = (0..8).into_iter().map(|i| 1 << i).collect();
        for i in 0..256 {
            u.num_set_bits[i] = match i {
                0 => 0,
                _ => {
                    for j in 0..8 {
                        if i as u8 & set_bit[j] != 0 {
                            u.first_set_bit[i] = j as i32;
                            break;
                        }
                    }
                    for j in 0..8 {
                        if i as u8 & set_bit[7 - j] != 0 {
                            u.last_set_bit[i] = (7 - j) as i32;
                            break;
                        }
                    }
                    1 + u.num_set_bits[i >> (u.first_set_bit[i] + 1)]
                }
            }
        }
        u
    }
    // pub fn get_first_set<K: Sized>(&self, val: K) -> i32 {
    //     self.first_set_bit[val as usize]
    // }
    // pub fn get_last_set<K: Sized>(&self, val: K) -> i32 {
    //     self.last_set_bit[val as usize]
    // }
    // pub fn get_num_set<K: Sized>(&self, val: K) -> K {
    //     self.num_set_bits[val as usize]
    // }
}

pub type InBktKey = u16;
pub type ValueType = [u8];

pub type KeyReminder<'a> = Option<&'a [u8]>;
#[repr(C, align(64))]
pub struct Bucket {
    pub head: ClIndex,
    pub curr_first_tms: u16,
    pub curr_last_tms: u16,
    // bloom_filter: [u8]
    // free entries list
    // num cl's
    //capacity
    //credit
    //lru counter
}

// impl Iterator for Bucket {
//     type Item = ();
//
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }
//pub struct
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum InsertRes {
    EntryExists((u32, u16)),
    Success((u32, u16)),
    OutOfSpace,
}

#[derive(Clone, Debug)]
pub enum FindRes<'a> {
    NotFound,
    Found(
        (
            ClSlot,
            ClIndex,
            CacheDataEntry,
            (Option<&'a [u8]>, Option<&'a [u8]>, Option<&'a [u8]>),
        ),
    ),
}
#[derive(Clone, Debug)]
pub enum DelRes {
    NotFound,
    Found(
        (
            ClSlot,
            ClIndex,
            CacheDataEntry,
            (Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>),
        ),
    ),
}
impl Bucket {
    pub fn new<C: CacheLine>() -> Bucket {
        Bucket {
            head: C::INVALID_CL,
            curr_first_tms: 0,
            curr_last_tms: 0,
        }
    }
    pub fn put<C: CacheLine>(
        &mut self,
        cl_store: &mut ClStore<C>,
        id: &[u8],
        value: Option<&[u8]>,
        bucket_key: InBktKey,
        key_reminder: KeyReminder,
        value_prefix: &ValueType,
        _should_evict: bool,
        new_cl: Option<ClIndex>,
    ) -> InsertRes {
        let mut cl = self.head;
        let mut cl_to_write = C::INVALID_CL;
        let mut slot_to_write = 0;
        let mut cl_tail = C::INVALID_CL;
        while cl != C::INVALID_CL {
            let cl_info = cl_store.get_cl_w_store(cl);
            let curr_cl = cl_info.0.unwrap();
            let res = curr_cl.find_entry_for_write(bucket_key);
            match self.check_condid(cl_info.1.unwrap(), key_reminder, res.condid) {
                Some(slot) => return InsertRes::EntryExists((cl, slot as u16)),
                None => {
                    cl_tail = cl;
                    if res.free != 0 {
                        cl_to_write = cl;
                        slot_to_write = res.free;
                    }
                    cl = curr_cl.get_next_cl()
                }
            }
        }
        let mut set_tail = false;
        if cl_to_write == C::INVALID_CL {
            match new_cl {
                Some(cl) => {
                    cl_to_write = cl;
                    slot_to_write = 1;
                    set_tail = true;
                }
                None => {}
            };
        }
        if cl_to_write != C::INVALID_CL {
            let mut slot = 0;
            loop {
                if slot_to_write & 0x1 != 0 {
                    break;
                }
                slot_to_write >>= 1;
                slot += 1;
            }
            let cl_info = cl_store.get_mut_cl_w_store(cl_to_write);
            cl_info
                .0
                .unwrap()
                .set_entry(slot as usize, bucket_key, value_prefix);

            cl_info.1.unwrap().set(slot, Some(id), key_reminder, value);
            if set_tail {
                cl_store
                    .get_mut_cl_w_store(cl_tail)
                    .0
                    .unwrap()
                    .set_next_cl(cl_to_write);
            }
            return InsertRes::Success((cl_to_write, slot as u16));
        }
        InsertRes::OutOfSpace
    }
    pub fn print<C: CacheLine>(&self, cl_store: &ClStore<C>) -> () {
        let mut cl = self.head;
        while cl != C::INVALID_CL {
            let clm = cl_store.get_cl_w_store(cl).0.unwrap();
            // println!("cl 0x{:x} : {}", cl, clm);
            cl = clm.get_next_cl();
        }
    }
    fn check_condid(
        &self,
        cl_store: &dyn PerClStore,
        key_reminder: KeyReminder,
        mut condids: ClCondidSlotsMask,
    ) -> Option<ClSlot> {
        let mut slot = 0;
        while condids != 0 {
            slot += 1;
            if condids.bitand(0x1) != 0 {
                if cl_store
                    .get_key_rem(slot - 1)
                    .unwrap_or_else(|| "".as_bytes())
                    .eq(key_reminder.unwrap())
                {
                    break;
                }
            }
            condids >>= 1;
        }
        if condids != 0 && slot != 0 {
            return Some(slot - 1);
        }
        None
    }
    pub fn get<'a, C: CacheLine>(
        &self,
        cl_store: &'a ClStore<C>,
        bucket_key: InBktKey,
        key_reminder: KeyReminder,
    ) -> FindRes<'a> {
        let mut cl = self.head;
        while cl != C::INVALID_CL {
            let cl_info = cl_store.get_cl_w_store(cl);
            let curr_cl = cl_info.0.unwrap();
            match self.check_condid(
                cl_info.1.unwrap(),
                key_reminder,
                curr_cl.find_entry_for_read(bucket_key).condid,
            ) {
                Some(slot) => {
                    return FindRes::Found((
                        slot,
                        cl,
                        curr_cl.get_entry(slot),
                        cl_info.1.unwrap().get_data(slot),
                    ))
                }
                None => cl = cl_info.0.unwrap().get_next_cl(),
            }
        }
        FindRes::NotFound
    }
    pub fn delete<C: CacheLine>(
        &mut self,
        cl_store: &mut ClStore<C>,
        bucket_key: InBktKey,
        key_reminder: KeyReminder,
    ) -> DelRes {
        let mut cl = self.head;
        while cl != C::INVALID_CL {
            let cl_info = cl_store.get_mut_cl_w_store(cl);
            let curr_cl = cl_info.0.unwrap();
            let curr_cl_info = cl_info.1.unwrap();
            match self.check_condid(
                curr_cl_info,
                key_reminder,
                curr_cl.find_entry_for_read(bucket_key).condid,
            ) {
                Some(slot) => {
                    let res =
                        DelRes::Found((slot, cl, curr_cl.get_entry(slot), curr_cl_info.free(slot)));
                    curr_cl.clear_entry(slot);
                    return res;
                }
                None => cl = curr_cl.get_next_cl(),
            }
        }
        DelRes::NotFound
    }
    //report capacity usage, entries+store info, on low usage migh get response to reduce quota
    //provide hit info and request capacity quota, calculate efficiency
}
#[cfg(test)]
mod tests {
    use crate::cache::bucket::Utility;

    #[test]
    fn test_utility() {
        let u = Utility::new();
        assert_eq!(u.first_set_bit[1 << 3], 3);
        assert_eq!(u.first_set_bit[1 << 7], 7);
        assert_eq!(u.last_set_bit[1 << 7], 7);
        assert_eq!(u.num_set_bits[0xff], 8);
        assert_eq!(u.num_set_bits[1 << 7], 1);
    }
    #[test]
    fn test_bucket() {}
}
