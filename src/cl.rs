use std::borrow::Borrow;
use std::ops::{BitAnd, BitXorAssign};
use crate::cl_store::ClIndex;

pub type ClValidSlotsMask = u8;

pub type ClSlot = usize;
pub type ValueType = u32;
pub type InBktKey = u16;

pub type KeyReminder = Option<Vec<u8>>;

//#[bitfield]
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug)]
pub struct CacheDataEntry {
    pub value: ValueType,
}
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug)]
pub struct CacheMetaEntry {
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


// #[bitmask(u8)]

// #[derive(Clone, Debug)]
// pub enum ClInsertResult {
//     NextCl(ClIndex),
//     AllocatedSlot(ClSlot),
// }
#[derive(Clone, Debug)]
pub enum ClFindResult {
    FoundWSlot((ClSlot, CacheDataEntry)),
    NotFountFreeSlotsAndNext((ClSlot, ClIndex)),
}
#[allow(dead_code)]
impl CacheLine {
    pub const INVALID_CL: u32 = u32::MAX as u32;
    pub const INVALID_SLOT: usize = 7;
    pub const NUM_SLOTS: usize = 7;
    pub fn new() -> CacheLine {
        CacheLine {
            entries: [CacheEntry {
                data_ent: CacheDataEntry { value: 0 },
            }; CacheLine::NUM_SLOTS],
            flags: CLFlags {
                valid_slots: 0x0,
                flags1: 0,
                flags2: 0,
            },
            next: CacheLine::INVALID_CL,
            bkt_keys: [0; CacheLine::NUM_SLOTS],
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
        let mut first_empty_slots = CacheLine::INVALID_SLOT;
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
            } else if first_empty_slots == CacheLine::INVALID_SLOT {
                first_empty_slots = i;
            }
        }
        ClFindResult::NotFountFreeSlotsAndNext((first_empty_slots, self.next))
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