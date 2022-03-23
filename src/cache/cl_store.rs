use crate::cache::cl::{CacheLine, ClSlot};
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
    fn free(&mut self, slot: ClSlot) -> (Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>);

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
    fn free(&mut self, slot: ClSlot) -> (Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>) {
        let f = |a: &mut Vec<Vec<u8>>| {
            if a.len() <= slot {
                None
            } else {
                let elem = Some(a[slot].to_vec());
                a[slot] = vec![];
                elem
            }
        };
        let ret = (f(&mut self.ids), f(&mut self.k_rems), f(&mut self.v_s));
        ret
    }

    fn set(
        &mut self,
        slot: ClSlot,
        id: Option<&[u8]>,
        k_rem: Option<&[u8]>,
        val_rem: Option<&[u8]>,
    ) -> () {
        let set_field = |b: Option<&[u8]>, a: &mut Vec<Vec<u8>>| match b {
            Some(id) => {
                if slot + 1 > a.len() {
                    a.resize(slot + 1, vec![]);
                }
                a[slot] = id.to_vec();
            }
            None => {
                if slot < a.len() {
                    a[slot] = vec![];
                }
            }
        };
        set_field(id, &mut self.ids);
        set_field(k_rem, &mut self.k_rems);
        set_field(val_rem, &mut self.v_s);
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
pub struct ClStore<C: CacheLine> {
    cls: Vec<C>,
    cls_store: Vec<Option<PerClVecMemStore>>,
    free_cls: RoaringBitmap,
    // n_cl_ents: u16,
}
#[allow(unused)]
impl<C: CacheLine> ClStore<C> {
    pub fn new(_num_cl_entries: u16) -> ClStore<C> {
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
                self.cls.push(C::new());
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
    pub fn get_cl(&mut self, cl_ix: ClIndex) -> Option<&C> {
        Some(self.cls.get(cl_ix as usize).unwrap())
    }
    #[inline(always)]
    pub fn get_cl_w_store(&self, cl_ix: ClIndex) -> (Option<&C>, Option<&dyn PerClStore>) {
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
    ) -> (Option<&mut C>, Option<&mut dyn PerClStore>) {
        (
            self.cls.get_mut(cl_ix as usize),
            match self.cls_store.get_mut(cl_ix as usize).unwrap() {
                Some(pcls) => Some(pcls),
                None => None,
            },
        )
    }
}
