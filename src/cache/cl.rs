use crate::cache::cl_store::ClIndex;
use std::cmp::min;
// use regex::internal::Input;
use std::fmt::{Display, Formatter};
// use std::fmt::{Display, Formatter};
use crate::cache::bucket::{InBktKey, ValueType};
use std::ops::{BitAnd, BitAndAssign, BitOrAssign, BitXor, BitXorAssign};

pub type ClSlotMask = u8;
pub type ClCondidSlotsMask = ClSlotMask;

pub type ClTakenSlotsMask = ClSlotMask;
pub type ClFreeSlotsMask = ClSlotMask;

pub type ClSlot = usize;
pub enum EntryType {
    Data,
    Metadata,
}
// pub enum ValueType {
//     Prefix([u8]),
//     Data([u8]),
//     StoreOffset([u8]),
//     ExtendedEntry([u8]),
// }
pub trait TCacheEntry {
    fn get_type(&self, slot: ClSlot) -> EntryType;
    fn get_value(&self, slot: ClSlot) -> ValueType;
    fn get_raw(&self, slot: ClSlot) -> Vec<u8>;
}
pub trait TCacheLine {
    const INVALID_CL: u32;
    const INVALID_SLOT: usize = 7;
    const NUM_SLOTS: usize = 7;
    const SLOTS_MASK: u8 = (1 << 7) - 1;
    const NUM_BYTES_INLINE_VAL: usize = 4;

    fn new() -> Self;
    fn new_with_entry(bucket_key: InBktKey, value: &ValueType) -> Self;
    fn set_next_cl(&mut self, next_cl: ClIndex) -> Option<ClIndex>;
    fn get_next_cl(&self) -> ClIndex;
    fn clear_entry(&mut self, slot: ClSlot) -> bool;
    fn get_entry(&self, offset: ClSlot) -> CacheDataEntry;
    fn set_entry(&mut self, offset: ClSlot, bucket_key: InBktKey, value: &ValueType) -> usize;
    fn find_entry_for_read(&self, bucket_key: InBktKey) -> ReadClFindResult;
    fn find_entry_for_write(&self, bucket_key: InBktKey) -> WriteClFindResult;
}
#[derive(Clone, Debug)]
pub struct ReadClFindResult {
    pub(crate) condid: ClCondidSlotsMask,
}

#[derive(Clone, Debug)]
pub struct WriteClFindResult {
    pub(crate) condid: ClCondidSlotsMask,
    pub(crate) free: ClFreeSlotsMask,
}
//#[bitfield]
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug)]
pub struct CacheDataEntry {
    pub value: [u8; CacheLine::NUM_BYTES_INLINE_VAL],
}
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug)]
pub struct CacheMetaEntry {
    blob: [u8; CacheLine::NUM_BYTES_INLINE_VAL],
}
//#[derive(BitfieldSpecifier)]
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub union CacheEntry {
    data_ent: CacheDataEntry,
    ctrl_ent: CacheMetaEntry,
}

// impl Display for CacheEntry {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:#?}", self.data_ent)
//     }
// }
//#[bitfield]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(C, align(4))]
pub struct CLFlags {
    valid_slots: ClTakenSlotsMask,
    flags1: u8,
    flags2: u16,
}
#[repr(C, align(64))]
// #[derive(Clone, Debug)]
pub struct CacheLine {
    flags: CLFlags,
    entries: [CacheEntry; CacheLine::NUM_SLOTS],
    bkt_keys: [InBktKey; CacheLine::NUM_SLOTS],
    next: ClIndex,
}
// use bitmask_enum::bitmask;

// #[bitmask(u8)]

// #[derive(Clone, Debug)]
// pub enum ClInsertResult {
//     NextCl(ClIndex),
//     AllocatedSlot(ClSlot),
// }

impl Display for CacheLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " Valid slots :{:b} flags1 0x{:x}, flags2 0x{:x}, next 0x{:x}\n",
            self.flags.valid_slots, self.flags.flags1, self.flags.flags2, self.next
        )?;
        unsafe {
            self.entries
                .iter()
                .enumerate()
                .for_each(|(i, e)| write!(f, " ent {} {:x?}", i, e.data_ent.value).unwrap());
        }
        write!(f, "\n")
    }
}
// #[allow(dead_code)]
// impl CacheLine {
//     pub fn new() -> CacheLine {
//         CacheLine {
//             entries: [CacheEntry {
//                 data_ent: CacheDataEntry {
//                     value: [0, 0, 0, 0],
//                 },
//             }; CacheLine::NUM_SLOTS],
//             flags: CLFlags {
//                 valid_slots: 0x0,
//                 flags1: 0,
//                 flags2: 0,
//             },
//             next: CacheLine::INVALID_CL,
//             bkt_keys: [0; CacheLine::NUM_SLOTS],
//         }
//     }
//     pub fn new_with_entry(bucket_key: InBktKey, value: &ValueType) -> CacheLine {
//         let mut cl = CacheLine::new();
//         cl.set_entry(0, bucket_key, value);
//         cl
//     }
// }

impl TCacheLine for CacheLine {
    const INVALID_CL: u32 = u32::MAX as u32;
    const INVALID_SLOT: usize = 7;
    const NUM_SLOTS: usize = 7;
    const SLOTS_MASK: u8 = (1 << 7) - 1;
    const NUM_BYTES_INLINE_VAL: usize = 4;
    fn new() -> CacheLine {
        CacheLine {
            entries: [CacheEntry {
                data_ent: CacheDataEntry {
                    value: [0, 0, 0, 0],
                },
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
    fn new_with_entry(bucket_key: InBktKey, value: &ValueType) -> CacheLine {
        let mut cl = CacheLine::new();
        cl.set_entry(0, bucket_key, value);
        cl
    }
    #[inline(always)]
    fn set_next_cl(&mut self, next_cl: ClIndex) -> Option<ClIndex> {
        let old_next = self.next;
        self.next = next_cl;
        match old_next {
            CacheLine::INVALID_CL => None,
            next => Some(next),
        }
    }
    #[inline(always)]
    fn get_next_cl(&self) -> ClIndex {
        self.next
    }
    fn clear_entry(&mut self, slot: ClSlot) -> bool {
        if self.flags.valid_slots.bitand(1 << slot) != 0 {
            self.flags.valid_slots.bitxor_assign(1 << slot);
            true
        } else {
            false
        }
    }
    #[inline(always)]
    fn get_entry(&self, offset: ClSlot) -> CacheDataEntry {
        unsafe { self.entries.get(offset).unwrap().data_ent }
    }

    #[inline(always)]
    fn set_entry(&mut self, offset: ClSlot, bucket_key: InBktKey, value: &ValueType) -> usize {
        let ent: &mut CacheEntry = self.entries.get_mut(offset).unwrap();

        let bkt_key_ptr: &mut InBktKey = self.bkt_keys.get_mut(offset).unwrap();
        *bkt_key_ptr = bucket_key;
        self.flags.valid_slots.bitxor_assign(1 << offset);
        let cp_len = min(CacheLine::NUM_BYTES_INLINE_VAL, value.len());
        unsafe {
            ent.data_ent.value[0..cp_len].copy_from_slice(&value[0..cp_len]);
        }
        cp_len
    }
    // it is assumed that either key reminder exists or not
    #[inline(always)]
    fn find_entry_for_read(&self, bucket_key: InBktKey) -> ReadClFindResult {
        let mut found_slot: ClCondidSlotsMask = 0x0;
        let mut curr_bit: ClCondidSlotsMask = 1;
        self.bkt_keys.iter().for_each(|&curr_bucket_key| {
            if curr_bucket_key == bucket_key {
                found_slot.bitor_assign(curr_bit);
            }
            curr_bit <<= 1;
        });
        found_slot.bitand_assign(self.flags.valid_slots);
        ReadClFindResult { condid: found_slot }
    }
    #[inline(always)]
    fn find_entry_for_write(&self, bucket_key: InBktKey) -> WriteClFindResult {
        let mut found_slot: ClCondidSlotsMask = 0x0;
        let mut curr_bit: ClCondidSlotsMask = 1;
        self.bkt_keys.iter().for_each(|&curr_bucket_key| {
            if curr_bucket_key == bucket_key {
                found_slot.bitor_assign(curr_bit);
            }
            curr_bit <<= 1;
        });
        found_slot.bitxor_assign(self.flags.valid_slots);
        WriteClFindResult {
            condid: found_slot,
            free: CacheLine::SLOTS_MASK.bitxor(self.flags.valid_slots),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::cl::CacheLine;
    use crate::cache::cl_store::TCacheLine;

    #[test]
    fn test_cl() {
        let value = (0..CacheLine::NUM_BYTES_INLINE_VAL * CacheLine::NUM_SLOTS)
            .into_iter()
            .map(|i| ((i * 11 % 255 + 7) % 255) as u8)
            .collect::<Vec<u8>>();
        //insert entries to cl
        let mut cl_test = CacheLine::new();
        assert_eq!(cl_test.get_next_cl(), CacheLine::INVALID_CL);
        cl_test.set_entry(0, 12, &value[0..CacheLine::NUM_BYTES_INLINE_VAL]);
        assert_eq!(cl_test.find_entry_for_read(12).condid, 0x1);
        cl_test.set_entry(1, 11, &value[0..CacheLine::NUM_BYTES_INLINE_VAL]);
        assert_eq!(cl_test.find_entry_for_read(11).condid, 0x1 << 1);
        //remove entry at slot 0
        assert_eq!(cl_test.clear_entry(0), true);
        //expect to not find any cond
        assert_eq!(cl_test.find_entry_for_read(12).condid, 0);
        cl_test.set_entry(0, 12, &value[0..CacheLine::NUM_BYTES_INLINE_VAL]);
        cl_test.set_entry(2, 12, &value[0..CacheLine::NUM_BYTES_INLINE_VAL]);
        cl_test.set_entry(3, 11, &value[0..CacheLine::NUM_BYTES_INLINE_VAL]);

        cl_test.set_entry(4, 12, &value[0..CacheLine::NUM_BYTES_INLINE_VAL]);
        assert_eq!(cl_test.find_entry_for_read(12).condid, 1 | 1 << 2 | 1 << 4);
        assert_eq!(cl_test.find_entry_for_read(11).condid, 1 << 1 | 1 << 3);
        //get entries in cl
        //set next
    }
}
