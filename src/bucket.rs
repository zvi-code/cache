use crate::cl::{CacheDataEntry, CacheLine, ClFindResult, ClSlot, InBktKey, KeyReminder, ValueType};
use crate::cl_store::ClIndex;

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
                    if first_empty_slots != CacheLine::INVALID_SLOT {
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
    pub fn delete(
        &mut self,
        cl_arr: &mut Vec<CacheLine>,
        bucket_key: InBktKey,
        key_reminder: &KeyReminder,
        key_reminders: Option<&[KeyReminder]>,
    ) -> FindRes {
        let mut cl = self.head;
        while cl != CacheLine::INVALID_CL {
            let res = cl_arr.get_mut(cl as usize).unwrap().remove_entry(
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
