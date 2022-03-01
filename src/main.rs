// use std::arch::x86_64::{_mm256_cmpeq_epi16, _mm256_shuffle_epi8, _mm_crc32_u64, _mm_sha1msg1_epu32};
use std::borrow::Borrow;
// use std::collections::{HashMap, HashSet, VecDeque};
// use std::error::Error;
// use std::intrinsics::offset;
use std::ops::{BitAnd, BitXorAssign};
// use std::ptr::hash;
// use modular_bitfield::prelude::*;

//Coding: local cache, for files, read\write, cache policy, assume no concarency
//Coding: given array of positive numbers, find the largest sum, such that you don’t add sum to adjustend nodes
//Code: function that prints all numbers that are sum of 2 qubes in 2 different way
//Coding: Given an array of numbers,
// using a moving window of size 3,
// report the maximum element within the window for each position.
//Coding: giver binary tree, transform it in place to dll left mid right order
//Coding: implement a fifo data structure
//Coding: local cache, for files, read\write, cache policy, assume no concarency
//Coding: given array of positive numbers, find the largest sum, such that you don’t add sum to adjustend nodes
//Coding: stream of int, find first non-repeating int
//Coding: you have event stream and start and end to each event, you need to print all the completed events…
fn main() {
    //insert Key, Value
    //get Key
    //Update Key
    //delete Key
    println!("Hello, world!");
    let mut cl_vec = vec![CacheLine::new()];
    let mut bucket = Bucket::new();
    bucket.head = 0;

    let kv_pairs = [
        (20, 199982),
        (32, 9982221),
        (45, 889292),
        (544, 8827272),
        (9, 8872),
        (22281, 88711),
    ];
    let insert_res = kv_pairs
        .iter()
        .map(|(k, v)| bucket.put(&mut cl_vec, *k, &None, *v, false, None))
        .collect::<Vec<InsertRes>>();

    let ret = bucket.get(&mut cl_vec, 15, &None, None);
    match ret {
        FindRes::Found(d) => {
            println!(
                "15 value={}: cl {} slot {} data {}",
                0xabba, d.1, d.0, d.2.value
            );
        }
        FindRes::NotFound => println!("didn't find entry"),
    }
    let ret = bucket.get(&mut cl_vec, 25, &None, None);
    match ret {
        FindRes::Found(d) => {
            println!("25: cl {} slot {} data {}", d.1, d.0, d.2.value);
        }
        FindRes::NotFound => println!("25: didn't find entry"),
    }
}

//#[bitfield]
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug)]
pub struct CacheDataEntry {
    // entry_type: B2,
    // valid: B1,
    // last: B1,
    // flags: u16,
    value: ValueType,
}
//#[bitfield]
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug)]
pub struct CacheMetaEntry {
    // entry_type: B2,
    // valid: B1,
    // last: B1,
    blob: ValueType,
}
//#[derive(BitfieldSpecifier)]
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub union CacheEntry {
    data_ent: CacheDataEntry,
    ctrl_ent: CacheMetaEntry,
}
//#[bitfield]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(C, align(4))]
pub struct CLFlags {
    valid_slots: ClValidSlotsMask,
    flags1: u8,
    flags2: u16,
}
#[repr(C, align(64))]
// #[derive(Debug)]
pub struct CacheLine {
    flags: CLFlags,
    entries: [CacheEntry; 7],
    bkt_keys: [InBktKey; 7],
    next: ClIndex,
}
// use bitmask_enum::bitmask;

type InBktKey = u16;

// #[bitmask(u8)]
type ClValidSlotsMask = u8;

type ClSlot = usize;
type ClIndex = u32;
type ValueType = u32;
type KeyReminder = Option<Vec<u8>>;
#[derive(Clone, Debug)]
pub enum ClInsertResult {
    NextCl(ClIndex),
    AllocatedSlot(ClSlot),
}
#[derive(Clone, Debug)]
pub enum ClFindResult {
    FoundWSlot((ClSlot, CacheDataEntry)),
    NotFountFreeSlotsAndNext((ClValidSlotsMask, ClIndex)),
}
impl CacheLine {
    pub const INVALID_CL: u32 = u32::MAX as u32;
    pub fn new() -> CacheLine {
        CacheLine {
            entries: [CacheEntry {
                data_ent: CacheDataEntry { value: 0 },
            }; 7],
            flags: CLFlags {
                valid_slots: 0x0,
                flags1: 0,
                flags2: 0,
            },
            next: CacheLine::INVALID_CL,
            bkt_keys: [0; 7],
        }
    }
    pub fn new_with_entry(bucket_key: InBktKey, value: ValueType) -> CacheLine {
        let mut cl = CacheLine::new();
        cl.insert_entry_to_slot(0, bucket_key, value);
        cl
    }
    pub fn set_next_cl(&mut self, next_cl: ClIndex) -> Option<ClIndex> {
        let old_next = self.next;
        self.next = next_cl;
        match old_next {
            CacheLine::INVALID_CL => None,
            next => Some(next),
        }
    }
    pub fn insert_entry_to_slot(
        &mut self,
        offset: ClSlot,
        bucket_key: InBktKey,
        value: ValueType,
    ) -> () {
        let ent: &mut CacheEntry = self.entries.get_mut(offset).unwrap();

        let bkt_key_ptr: &mut InBktKey = self.bkt_keys.get_mut(offset).unwrap();
        *bkt_key_ptr = bucket_key;
        self.flags.valid_slots.bitxor_assign(1 << offset);
        // unsafe {
        ent.data_ent.value = value; //u32::from_be_bytes(*value[0..4]);
                                    // }
                                    // ent
    }
    // it is assumed that either key reminder exists or not
    pub fn find_entry(
        &self,
        bucket_key: InBktKey,
        key_reminder: &KeyReminder,
        key_reminders: Option<&[KeyReminder]>,
    ) -> ClFindResult {
        let mut first_empty_slots = 0xff;
        for (i, bktk) in self.bkt_keys.iter().enumerate() {
            if (self.flags.valid_slots.bitand(1 << i)) != 0 {
                if *bktk == bucket_key {
                    let mut rem_cmp = &None;
                    match key_reminders {
                        Some(key_reminders) => rem_cmp = &key_reminders[i],
                        None => (),
                    }
                    if *key_reminder == *rem_cmp {
                        return ClFindResult::FoundWSlot((i as ClSlot, unsafe {
                            self.entries.get(i).unwrap().data_ent.clone()
                        }));
                    }
                }
            } else if first_empty_slots == 0xff {
                first_empty_slots = i;
            }
        }
        ClFindResult::NotFountFreeSlotsAndNext((first_empty_slots as ClValidSlotsMask, self.next))
    }
    pub fn remove_entry(
        &mut self,
        bucket_key: InBktKey,
        key_reminder: &KeyReminder,
        key_reminders: Option<&[KeyReminder]>,
    ) -> ClFindResult {
        let res = self.find_entry(bucket_key, key_reminder, key_reminders);
        match res.borrow() {
            ClFindResult::FoundWSlot((slot, _entry)) => {
                self.flags.valid_slots.bitxor_assign(1 << slot)
            }
            _ => (),
        };
        res
    }
}

#[repr(C, align(64))]
pub struct Bucket {
    head: ClIndex,
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
        cl_arr: &mut Vec<CacheLine>,
        bucket_key: InBktKey,
        key_reminder: &KeyReminder,
        value: ValueType,
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
            let res =
                cl_arr
                    .get_mut(cl as usize)
                    .unwrap()
                    .find_entry(bucket_key, key_reminder, None);
            cl_tail = cl;
            match res {
                ClFindResult::FoundWSlot((_slot, _d)) => return InsertRes::EntryExists(cl),
                ClFindResult::NotFountFreeSlotsAndNext((first_empty_slots, next_cl)) => {
                    if first_empty_slots != 0xff {
                        cl_to_write = cl;
                        slot_to_write = first_empty_slots;
                    }
                    cl = next_cl;
                }
            }
        }
        if cl_to_write != CacheLine::INVALID_CL {
            cl_arr
                .get_mut(cl_to_write as usize)
                .unwrap()
                .insert_entry_to_slot(slot_to_write as usize, bucket_key, value);
            return InsertRes::Success(cl_to_write);
        }
        match new_cl {
            Some(cl) => {
                cl_arr
                    .get_mut(cl as usize)
                    .unwrap()
                    .insert_entry_to_slot(0, bucket_key, value);
                cl_arr.get_mut(cl_tail as usize).unwrap().set_next_cl(cl);
                return InsertRes::Success(cl);
            }
            None => (),
        }
        InsertRes::OutOfSpace
    }
    pub fn get(
        &mut self,
        cl_arr: &mut Vec<CacheLine>,
        bucket_key: InBktKey,
        key_reminder: &KeyReminder,
        key_reminders: Option<&[KeyReminder]>,
    ) -> FindRes {
        let mut cl = self.head;
        while cl != CacheLine::INVALID_CL {
            let res = cl_arr.get_mut(cl as usize).unwrap().find_entry(
                bucket_key,
                key_reminder,
                key_reminders,
            );
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
// pub struct Cache <K>{
//     hmap: HashMap<K, Bucket>,
//     hashs: HashSet<u32>,
//     cls: Vec<CacheLine>,
//     //free CL's
//     free_cls: VecDeque<u32>,
// }
// impl Cache<u32> {
//     fn get_bucket_id(id: &[u8]) -> u32 {
//         u32::from_be_bytes(id[0..4].try_into().unwrap())
//         //_mm_crc32_u64()
//         //hash()
//         //_mm256_shuffle_epi8()
//
//     }
//     pub fn upsert(&mut self, key: &[u8], value: &[u8]) -> Option<bool>{
//         let bucket_id = self.get_bucket_id(key);
//         //lock bucket
//         let bucket = self.hmap.get_mut(&bucket_id);
//         let bkt = match bucket {
//             Some(bkt_list) => {
//                 bkt_list
//             },
//             None => {
//                 Bucket::new()
//             }
//         };
//
//         None
//     }
//
// }
