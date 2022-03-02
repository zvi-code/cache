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