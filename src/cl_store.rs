use crate::cl::ClSlot;
use crate::CacheLine;
// use plotters::coord::ranged1d::ReversibleRanged;
use roaring::RoaringBitmap;

/// Store data associated to the entry in the corresponding cl
/// the index is direct mapping from cl_ix+offset in cl
/// the store could be in memory, or any other type of storage
/// the basic types of data:
///     key reminder,
///     id,
///     data+type(the data could be by itself pointer to other data source,
///         for example
///             it could be an bucket+offset in S3 where the actual data resides)

/// to do add support for fixed size value opt, and fixed size key support

pub type ClIndex = u32;

pub trait PerClStore {
    fn free(&mut self, slot: ClSlot) -> ();

    fn set(
        &mut self,
        slot: ClSlot,
        id: Option<&[u8]>,
        k_rem: Option<&[u8]>,
        val_rem: Option<&[u8]>,
    ) -> ();
    fn get_data(&self, slot: ClSlot) -> (Option<&[u8]>, Option<&[u8]>, Option<&[u8]>);
    fn get_id(&self, slot: ClSlot) -> Option<&[u8]>;
    fn get_key_rem(&self, slot: ClSlot) -> Option<&[u8]>;
    fn get_value_suffix(&self, slot: ClSlot) -> Option<&[u8]>;
}
pub struct PerClVecMemStore {
    ids: Vec<Vec<u8>>,
    k_rems: Vec<Vec<u8>>,
    v_s: Vec<Vec<u8>>,
}

impl PerClVecMemStore {
    pub fn new() -> PerClVecMemStore {
        PerClVecMemStore {
            ids: vec![],
            k_rems: vec![],
            v_s: vec![],
        }
    }
}

impl PerClStore for PerClVecMemStore {
    fn free(&mut self, slot: ClSlot) -> () {
        self.ids[slot] = vec![];
        self.k_rems[slot] = vec![];
        self.v_s[slot] = vec![];
    }

    fn set(
        &mut self,
        slot: ClSlot,
        id: Option<&[u8]>,
        k_rem: Option<&[u8]>,
        val_rem: Option<&[u8]>,
    ) -> () {
        match id {
            Some(id) => {
                if slot + 1 > self.ids.len() {
                    self.ids.resize(slot + 1, vec![]);
                }
                self.ids[slot] = id.to_vec();
            }
            None => {
                if slot < self.ids.len() {
                    self.ids[slot] = vec![];
                }
            }
        }
        match k_rem {
            Some(k_rem) => {
                if slot + 1 > self.k_rems.len() {
                    self.k_rems.resize(slot + 1, vec![]);
                }
                self.k_rems[slot] = k_rem.to_vec();
            }
            None => {
                if slot < self.k_rems.len() {
                    self.k_rems[slot] = vec![];
                }
            }
        }
        match val_rem {
            Some(val_rem) => {
                if slot + 1 > self.v_s.len() {
                    self.v_s.resize(slot + 1, vec![]);
                }
                self.v_s[slot] = val_rem.to_vec();
            }
            None => {
                if slot < self.v_s.len() {
                    self.v_s[slot] = vec![];
                }
            }
        }
    }
    fn get_data(&self, slot: ClSlot) -> (Option<&[u8]>, Option<&[u8]>, Option<&[u8]>) {
        (
            self.get_id(slot),
            self.get_key_rem(slot),
            self.get_value_suffix(slot),
        )
    }
    fn get_id(&self, slot: ClSlot) -> Option<&[u8]> {
        match self.ids.get(slot) {
            Some(k_rems) => {
                if *k_rems != vec![] {
                    return Some(k_rems.as_slice());
                } else {
                    None
                }
            }
            None => None,
        }
    }
    fn get_key_rem(&self, slot: ClSlot) -> Option<&[u8]> {
        match self.k_rems.get(slot) {
            Some(k_rems) => {
                if *k_rems != vec![] {
                    return Some(k_rems.as_slice());
                } else {
                    None
                }
            }
            None => None,
        }
    }
    fn get_value_suffix(&self, slot: ClSlot) -> Option<&[u8]> {
        match self.v_s.get(slot) {
            Some(k_rems) => {
                if *k_rems != vec![] {
                    return Some(k_rems.as_slice());
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
pub struct ClStore {
    cls: Vec<CacheLine>,
    cls_store: Vec<Option<PerClVecMemStore>>,
    free_cls: RoaringBitmap,
    // n_cl_ents: u16,
}
#[allow(unused)]
impl ClStore {
    pub fn new(_num_cl_entries: u16) -> ClStore {
        ClStore {
            cls: vec![],
            free_cls: RoaringBitmap::new(),
            // n_cl_ents: num_cl_entries,
            cls_store: vec![],
        }
    }
    pub fn allocate_cl(&mut self) -> ClIndex {
        match self.free_cls.min() {
            Some(cl_ix) => {
                match self.cls_store[cl_ix as usize] {
                    Some(_) => (),
                    None => self.cls_store[cl_ix as usize] = Some(PerClVecMemStore::new()),
                };
                return cl_ix;
            }
            None => {
                self.cls.push(CacheLine::new());
                self.cls_store.push(Some(PerClVecMemStore::new()));
                (self.cls.len() - 1) as ClIndex
            }
        }
    }
    pub fn delete_cl(&mut self, cl_ix: ClIndex) -> () {
        match self.cls_store.get(cl_ix as usize) {
            Some(_) => {
                self.cls_store[cl_ix as usize] = None;
                self.free_cls.insert(cl_ix);
            }
            None => (),
        }
    }
    #[inline(always)]
    pub fn get_cl(&mut self, cl_ix: ClIndex) -> Option<&CacheLine> {
        Some(self.cls.get(cl_ix as usize).unwrap())
    }
    #[inline(always)]
    pub fn get_cl_w_store(
        &mut self,
        cl_ix: ClIndex,
    ) -> (Option<&CacheLine>, Option<&dyn PerClStore>) {
        (
            self.cls.get(cl_ix as usize),
            match self.cls_store.get(cl_ix as usize).unwrap() {
                Some(pcls) => Some(pcls),
                None => None,
            },
        )
    }
    #[inline(always)]
    pub fn get_mut_cl_w_store(
        &mut self,
        cl_ix: ClIndex,
    ) -> (Option<&mut CacheLine>, Option<&mut dyn PerClStore>) {
        (
            self.cls.get_mut(cl_ix as usize),
            match self.cls_store.get_mut(cl_ix as usize).unwrap() {
                Some(pcls) => Some(pcls),
                None => None,
            },
        )
    }
}
