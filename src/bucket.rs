use crate::cl::{
    CacheDataEntry, CacheLine, ClFindResult, ClSlot, InBktKey, KeyReminder, ValueType,
};
use crate::cl_store::ClIndex;
use crate::ClStore;

#[repr(C, align(64))]
pub struct Bucket {
    pub head: ClIndex,
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
    EntryExists(u32),
    Success(u32),
    OutOfSpace,
}
#[derive(Clone, Debug)]
pub enum FindRes {
    NotFound,
    Found((ClSlot, ClIndex, CacheDataEntry)),
}
impl Bucket {
    pub fn new() -> Bucket {
        Bucket {
            head: CacheLine::INVALID_CL,
        }
    }
    pub fn put(
        &mut self,
        cl_store: &mut ClStore,
        id: &[u8],
        value: Option<&[u8]>,
        bucket_key: InBktKey,
        key_reminder: KeyReminder,
        value_prefix: ValueType,
        _should_evict: bool,
        new_cl: Option<ClIndex>,
    ) -> InsertRes {
        //get first cl in bucket - if exist
        //search entry in cl - remember free slots
        //if not found continue to next cl
        //if not found insert new entry to free slot or allocate new cl
        //evict entry if can't allocate any more resources
        let mut cl = self.head;
        let mut cl_to_write = CacheLine::INVALID_CL;
        let mut slot_to_write = 0;
        let mut cl_tail = CacheLine::INVALID_CL;
        while cl != CacheLine::INVALID_CL {
            let cl_info = cl_store.get_cl_w_store(cl);
            let res = cl_info
                .0
                .unwrap()
                .find_entry(bucket_key, key_reminder, cl_info.1.unwrap());
            cl_tail = cl;
            match res {
                ClFindResult::FoundWSlot((_slot, _d)) => return InsertRes::EntryExists(cl),
                ClFindResult::NotFountFreeSlotsAndNext((first_empty_slots, next_cl)) => {
                    if first_empty_slots != CacheLine::INVALID_SLOT {
                        cl_to_write = cl;
                        slot_to_write = first_empty_slots;
                    }
                    cl = next_cl;
                }
            }
        }
        let mut set_tail = false;
        if cl_to_write == CacheLine::INVALID_CL {
            match new_cl {
                Some(cl) => {
                    cl_to_write = cl;
                    slot_to_write = 0;
                    set_tail = true;
                }
                None => {}
            };
        }
        if cl_to_write != CacheLine::INVALID_CL {
            let cl_info = cl_store.get_mut_cl_w_store(cl_to_write);
            cl_info.0.unwrap().insert_entry_to_slot(
                slot_to_write as usize,
                bucket_key,
                value_prefix,
            );
            cl_info
                .1
                .unwrap()
                .set(slot_to_write, Some(id), Some(&key_reminder.unwrap()), value);
            if set_tail {
                cl_store
                    .get_mut_cl_w_store(cl_tail)
                    .0
                    .unwrap()
                    .set_next_cl(cl_to_write);
            }
            return InsertRes::Success(cl_to_write);
        }
        InsertRes::OutOfSpace
    }
    pub fn get(
        &mut self,
        cl_store: &mut ClStore,
        bucket_key: InBktKey,
        key_reminder: KeyReminder,
    ) -> FindRes {
        let mut cl = self.head;
        while cl != CacheLine::INVALID_CL {
            let cl_info = cl_store.get_cl_w_store(cl);
            let res = cl_info
                .0
                .unwrap()
                .find_entry(bucket_key, key_reminder, cl_info.1.unwrap());
            match res {
                ClFindResult::FoundWSlot((slot, data)) => return FindRes::Found((slot, cl, data)),
                ClFindResult::NotFountFreeSlotsAndNext((_first_empty_slots, next_cl)) => {
                    cl = next_cl;
                }
            }
        }
        FindRes::NotFound
    }
    pub fn delete(
        &mut self,
        cl_store: &mut ClStore,
        bucket_key: InBktKey,
        key_reminder: KeyReminder,
    ) -> FindRes {
        let mut cl = self.head;
        while cl != CacheLine::INVALID_CL {
            let cl_info = cl_store.get_mut_cl_w_store(cl);
            let res = cl_info
                .0
                .unwrap()
                .remove_entry(bucket_key, key_reminder, cl_info.1.unwrap());
            match res {
                ClFindResult::FoundWSlot((slot, data)) => return FindRes::Found((slot, cl, data)),
                ClFindResult::NotFountFreeSlotsAndNext((_first_empty_slots, next_cl)) => {
                    cl = next_cl;
                }
            }
        }
        FindRes::NotFound
    }
}
