use crate::cl::{InBktKey, ValueType};
use crate::cl_store::ClIndex;
use crate::{Bucket, CacheLine, ClStore, FindRes, InsertRes};
use murmur3::murmur3_x86_128;
use std::borrow::Borrow;

pub type IBktId = usize;
#[allow(unused)]
pub struct Cache {
    buckets: Vec<Bucket>,
    cl_store: ClStore,
    next_free_cl: ClIndex,
    num_buckets: IBktId,
    bytes_for_bucket_id: usize,
    inline_val_num_bytes: u32,
    inline_key_num_bytes: u32,
    total_capacity: u32,
}
#[allow(unused)]
impl Cache {
    pub fn new(bytes_for_bucket_id: usize, capacity: u32) -> Cache {
        let mut cache = Cache {
            buckets: vec![],
            cl_store: ClStore::new(7),
            next_free_cl: CacheLine::INVALID_CL,
            num_buckets: 1 << (bytes_for_bucket_id * 8),
            bytes_for_bucket_id,
            inline_val_num_bytes: 4,
            inline_key_num_bytes: 2,
            total_capacity: capacity,
        };
        (0..cache.num_buckets).for_each(|_| cache.buckets.push(Bucket::new()));
        cache.next_free_cl = cache.cl_store.allocate_cl();
        cache
    }
    fn get_bucket_id(&self, id: &[u8]) -> (IBktId, InBktKey, u128) {
        let hash = murmur3_x86_128(&mut id.clone(), 0).unwrap();
        (
            IBktId::from_be_bytes(
                hash.to_be_bytes()[0..self.bytes_for_bucket_id]
                    .try_into()
                    .unwrap(),
            ),
            InBktKey::from_be_bytes(
                hash.to_be_bytes()[0..self.inline_key_num_bytes as usize]
                    .try_into()
                    .unwrap(),
            ),
            hash,
        )
    }
    pub fn upsert(&mut self, key: &[u8], value: &[u8]) -> bool {
        let (bucket_id, bkt_key, h) = self.get_bucket_id(key);
        //lock bucket
        let res = self.buckets[bucket_id].put(
            &mut self.cl_store,
            key,
            Some(value),
            bkt_key,
            Some(&h.to_be_bytes()),
            ValueType::from_be_bytes(
                (&value[0..self.inline_val_num_bytes as usize])
                    .try_into()
                    .unwrap(),
            ),
            false,
            Some(self.next_free_cl),
        );
        match res.borrow() {
            InsertRes::Success(cl) => {
                if *cl == self.next_free_cl {
                    self.next_free_cl = self.cl_store.allocate_cl();
                }
                return true;
            }
            _ => (),
        };

        false
    }
    pub fn get(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        let (bucket_id, bkt_key, h) = self.get_bucket_id(key);
        //lock bucket
        //need to add id to get, can't rely on hash
        let res = self.buckets[bucket_id].get(&mut self.cl_store, bkt_key, Some(&h.to_be_bytes()));
        match res {
            FindRes::Found(d) => {
                println!(
                    "Get: key={:?}: cl {} slot {} data {}",
                    key, d.1, d.0, d.2.value
                );
                Some(d.2.value.to_be_bytes().to_vec())
            }
            FindRes::NotFound => {
                println!("Get: key={:?}: didn't find entry", key);
                None
            }
        }
    }
}
