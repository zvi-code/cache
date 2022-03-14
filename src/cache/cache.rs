use crate::cache::bucket::{Bucket, FindRes, InsertRes};
use crate::cache::cl::{CacheLine, InBktKey};
use crate::cache::cl_store::{ClIndex, ClStore};
use murmur3::murmur3_x86_128;
use std::borrow::Borrow;

pub type IBktId = usize;
#[allow(unused)]
pub struct Cache {
    buckets: Vec<Bucket>,
    cl_store: ClStore,
    next_free_cl: ClIndex,
    num_buckets: IBktId,
    buckets_mask: u128,
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
            buckets_mask: 0,
            bytes_for_bucket_id,
            inline_val_num_bytes: 4,
            inline_key_num_bytes: 2,
            total_capacity: capacity,
        };
        (0..bytes_for_bucket_id).for_each(|_| {
            cache.buckets_mask <<= 8;
            cache.buckets_mask |= 0xff;
        });
        (0..cache.num_buckets).for_each(|_| {
            let mut bkt = Bucket::new();
            bkt.head = cache.cl_store.allocate_cl();
            cache.buckets.push(bkt)
        });
        cache.next_free_cl = cache.cl_store.allocate_cl();
        cache
    }
    #[inline(always)]
    fn get_bucket_id(&self, id: &[u8]) -> (IBktId, InBktKey, u128) {
        let hash = murmur3_x86_128(&mut id.clone(), 0).unwrap();
        (
            (hash & self.buckets_mask) as IBktId,
            InBktKey::from_be_bytes(
                hash.to_be_bytes()[0..self.inline_key_num_bytes as usize]
                    .try_into()
                    .unwrap(),
            ),
            hash,
        )
    }
    pub fn print_bucket(&mut self, key: &[u8]) -> IBktId {
        let (bucket_id, bkt_key, h) = self.get_bucket_id(key);
        self.buckets[bucket_id].print(&self.cl_store);
        bucket_id
    }
    pub fn upsert(&mut self, key: &[u8], value: &[u8]) -> bool {
        let (bucket_id, bkt_key, h) = self.get_bucket_id(key);
        //lock bucket
        let mut num_inline_value_bytes = value.len();
        if num_inline_value_bytes > self.inline_val_num_bytes as usize {
            num_inline_value_bytes = self.inline_val_num_bytes as usize;
        }
        let res = self.buckets[bucket_id].put(
            &mut self.cl_store,
            key,
            Some(value),
            bkt_key,
            Some(&h.to_be_bytes()),
            &value[0..num_inline_value_bytes],
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
            FindRes::Found(d) => Some(d.2.value.to_vec()),
            FindRes::NotFound => None,
        }
    }
}
