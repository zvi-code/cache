// // use crate::observability::rocksdb::{flush_metrics, flush_stats};
// use crate::performance::data_loader::DataLoader;
// // use crate::storage::db::{RAW_VECTORS_COL, SYSTEM_COL};
// // use crate::storage::store_config::{RocksDBConfig, StoreMode};
// // use crate::storage::store_load_config::{store_override_cf_config, store_override_db_config};
// use byte_slice_cast::AsByteSlice;
// use rand::{thread_rng, Rng};
// // use rocksdb::{
// //     ColumnFamilyDescriptor, Error, FlushOptions, IteratorMode, Options, ReadOptions, WriteBatch,
// //     DB as RocksDB,
// // };
// use std::cmp::min;
// use std::env;
// use std::io::{BufWriter, Write};
// use std::path::PathBuf;
// use std::time::Instant;
//
// const KB: usize = 1024;
//
// const _NANO_IN_SEC: usize = 1_000_000_000;
// const MICRO_IN_SEC: usize = 1_000_000;
// const _MILI_IN_SEC: usize = 1_000_000;
// const PERIODIC_PRINT_INTERVAL_PERCENT: u32 = 1;
// const _PERIODIC_PRINT_INTERVAL_SEC: u32 = 1;
//
// pub struct RandomDataLoad {
//     // options: Options,
//     db_path: PathBuf,
//     // db: RocksDB,
//     expected_num_vectors: usize,
//     curr_num_vectors: usize,
//     cache_size_mb: u32,
//     vec_len_u8: usize,
//     // cf_name: String,
//     name: String,
// }
// impl RandomDataLoad {
//     fn get_db_path(name: &str, config_id: u64, version: u32) -> PathBuf {
//         PathBuf::from(&*format!(
//             "{}/{}_cfg_digest_{}.ver_{}",
//             env::temp_dir().display(),
//             name,
//             config_id.to_string(),
//             version.to_string()
//         ))
//     }
//     #[allow(unused)]
//     pub fn new<W: Write>(
//         // mode: StoreMode,
//         config_id: u64,
//         sstbl_factory: Option<String>,
//         num_vectors: usize,
//         dim: usize,
//         mmap: u8,
//         direct: u8,
//         cache_size_mb: u32,
//         writer: &mut BufWriter<W>,
//     ) -> RandomDataLoad {
//         let curr_num_vecs: u32;
//         // store_override_cf_config("raw_vectors", "table_factory", sstbl_factory);
//         // store_override_cf_config(
//         //     "raw_vectors",
//         //     "cache_size_mb",
//         //     Some(cache_size_mb.to_string()),
//         // );
//         // store_override_db_config("use_mmap_mask", Some(mmap.to_string()));
//         // store_override_db_config("use_direct_mask", Some(direct.to_string()));
//         //
//         // let db_config = RocksDBConfig::new(mode);
//         // writer.write(db_config.config_string.as_ref()).unwrap();
//         let mut db_ver = 0;
//         let mut db_path = RandomDataLoad::get_db_path("db_perf_synth_data", config_id, db_ver);
//
//         // let mut db;
//         // let mut db_info: (u32, bool);
//         // loop {
//         //     let cf_dsc: Vec<ColumnFamilyDescriptor> = db_config
//         //         .cf_options
//         //         .iter()
//         //         .map(|(name, cf_op)| ColumnFamilyDescriptor::new(name, cf_op.clone()))
//         //         .collect();
//         //     db = RocksDB::open_cf_descriptors(&db_config.options, &db_path, cf_dsc).unwrap();
//         //     db_info = RandomDataLoad::is_db_valid(&db);
//         //     if db_info.1 && (db_info.0 <= num_vectors as u32) {
//         //         curr_num_vecs = db_info.0;
//         //         writer
//         //             .write(
//         //                 format!(
//         //                     "\n{:?}:reopened db {} num vectors already exist {}\n",
//         //                     chrono::offset::Utc::now(),
//         //                     db_path.to_str().unwrap(),
//         //                     curr_num_vecs
//         //                 )
//         //                 .as_ref(),
//         //             )
//         //             .unwrap();
//         //         break;
//         //     } else {
//         //         let _ = RocksDB::destroy(&db_config.options, &db_path);
//         //     }
//         //
//         //     if !db_info.1 {
//         //         db_ver += 1;
//         //         db_path = RandomDataLoad::get_db_path("db_perf_synth_data", config_id, db_ver);
//         //     }
//         // }
//
//         RandomDataLoad {
//             // options: db_config.options.clone(),
//             db_path,
//             // db,
//             expected_num_vectors: num_vectors,
//             curr_num_vectors: curr_num_vecs as usize,
//             cache_size_mb,
//             vec_len_u8: dim * 4,
//             // cf_name: RAW_VECTORS_COL.to_string(),
//             name: &*format!(".{}.{}", cache_size_mb, config_id),
//         }
//     }
//     // fn is_db_valid(db: &RocksDB) -> (u32, bool) {
//     //     let cf_ref = db.cf_handle(SYSTEM_COL).unwrap();
//     //
//     //     let info = db.get_cf(cf_ref, "db_shutdown");
//     //     let (existed, valid): (bool, bool) = match info {
//     //         Ok(op) => match op {
//     //             Some(val) => {
//     //                 if val.len() != 0 {
//     //                     (true, unsafe {
//     //                         String::from_utf8_unchecked(val).parse().unwrap()
//     //                     })
//     //                 } else {
//     //                     (false, false)
//     //                 }
//     //             }
//     //             None => (false, false),
//     //         },
//     //         Err(_e) => (false, false),
//     //     };
//     //
//     //     if existed && valid {
//     //         (
//     //             {
//     //                 let num_str = db.get_cf(cf_ref, "num_entries").unwrap().unwrap();
//     //                 usize::from_be_bytes(num_str.as_slice().try_into().unwrap()) as u32
//     //             },
//     //             true,
//     //         )
//     //     } else {
//     //         (0, !existed)
//     //     }
//     // }
//     #[allow(dead_code)]
//     pub fn flush_metrics<W: Write>(&self, writer: &mut BufWriter<W>) {
//         // flush_stats(&self.options, writer);
//         // flush_metrics(&self.db, RAW_VECTORS_COL, writer);
//     }
//     #[allow(dead_code)]
//     pub fn db_gracefull_shutdown<W: Write>(&self, writer: &mut BufWriter<W>) -> () {
//         // let cf_ref = self.db.cf_handle(SYSTEM_COL).unwrap();
//         // let res = self
//         //     .db
//         //     .put_cf(cf_ref, "num_entries", self.curr_num_vectors.to_be_bytes());
//         // if !res.is_ok() {
//         //     panic!("set num_entries failed Err = {}", res.unwrap_err());
//         // }
//         // let res = self.db.put_cf(cf_ref, "db_shutdown", "true");
//         // if !res.is_ok() {
//         //     panic!("set db_shutdown failed Err = {}", res.unwrap_err());
//         // }
//         // self.flush_writes(SYSTEM_COL, writer);
//         // self.flush_writes(&*self.cf_name, writer);
//         // writer
//         //     .write(
//         //         format!(
//         //             "\n{:?}:db_gracefull_shutdown num vectors {}\n",
//         //             chrono::offset::Utc::now(),
//         //             self.curr_num_vectors
//         //         )
//         //         .as_ref(),
//         //     )
//         //     .unwrap();
//         // self.db.cancel_all_background_work(true);
//     }
//     pub fn db_start_work<W: Write>(&self, writer: &mut BufWriter<W>) -> () {
//         // let cf_ref = self.db.cf_handle(SYSTEM_COL).unwrap();
//         // let res = self.db.put_cf(cf_ref, "db_shutdown", "false");
//         // if !res.is_ok() {
//         //     panic!("set db_shutdown failed Err = {}", res.unwrap_err());
//         // }
//         // self.flush_writes(SYSTEM_COL, writer);
//     }
//     pub fn destroy<W: Write>(&self, writer: &mut BufWriter<W>) -> () {
//         // //cleanup the db
//         // self.flush_writes(&*self.cf_name, writer);
//         // let _ = RocksDB::destroy(&self.options, &self.db_path);
//     }
//     pub fn get_range(&self) -> u32 {
//         self.curr_num_vectors as u32
//     }
//
//     ///based on expected hit ratio, calculate the max vector id we should run load on..
//     pub fn get_range_for_cache_hit_expect(&self, cache_hit: usize) -> u32 {
//         let vecs_in_cache = 9 * self.cache_size_mb as usize * 1024 * 1024 / (10 * self.vec_len_u8);
//         (if cache_hit != 0 {
//             min((vecs_in_cache * 100) / cache_hit, self.expected_num_vectors)
//         } else {
//             self.expected_num_vectors
//         }) as u32
//     }
//     fn set_ids_data<W: Write>(
//         &self,
//         op_key_suffix: Option<&str>,
//         ids_to_set: &[usize],
//         with_wal: bool,
//         flush: bool,
//         writer: &mut BufWriter<W>,
//     ) -> usize {
//         let mut vector = vec![0_u8; self.vec_len_u8];
//
//         for i in 0..self.vec_len_u8 {
//             vector[i] = rand::random::<u8>();
//         }
//         let cf_ref = self.db.cf_handle(&*self.cf_name).unwrap();
//         let start = Instant::now();
//         // let mut batch = WriteBatch::default();
//         for num_in_batch in ids_to_set {
//             for (j, x) in num_in_batch.to_be_bytes().into_iter().enumerate() {
//                 vector[j] = x;
//             }
//             // batch.put_cf(
//             //     cf_ref,
//             //     self.build_key(op_key_suffix, *num_in_batch),
//             //     vector.to_vec().as_byte_slice(),
//             // );
//         }
//         // let res = if with_wal {
//         //     self.db.write(batch) // Atomically commits the batch
//         // } else {
//         //     self.db.write_without_wal(batch) // Atomically commits the batch
//         // };
//         // if !res.is_ok() {
//         //     panic!("write failed Err = {}", res.unwrap_err());
//         // }
//         let dur = (Instant::now() - start).as_micros() as usize;
//         // if flush {
//         //     self.flush_writes(&*self.cf_name, writer);
//         // }
//         dur
//     }
//     fn get_ids_data(
//         &self,
//         // ro: &mut ReadOptions,
//         op_key_suffix: Option<&str>,
//         get_ids: &[usize],
//         checksum: bool,
//         fill_cache: bool,
//         verify_data: bool,
//     ) -> (usize, u32) {
//         let start = Instant::now();
//
//         // let cf_ref = self.db.cf_handle(&*self.cf_name).unwrap();
//         //
//         // let cf_ids = get_ids
//         //     .into_iter()
//         //     .map(|id| (cf_ref, self.build_key(op_key_suffix, *id)));
//         //
//         // ro.set_verify_checksums(checksum);
//         // ro.fill_cache(fill_cache);
//         // let results = self.db.multi_get_cf_opt(cf_ids, &ro);
//         // assert_eq!(results.len(), get_ids.len());
//         let dur = (Instant::now() - start).as_micros() as usize;
//         (
//             dur,
//             if verify_data {
//                 //self.verify_read_res(results, get_ids)
//                 0
//             } else {
//                 0
//             },
//         )
//     }
//
//     fn flush_writes<W: Write>(&self, name: &str, writer: &mut BufWriter<W>) -> () {
//         // let cf_ref = self.db.cf_handle(name).unwrap();
//         // //todo in case of memtbl we want to avoid the flush actually
//         // let mut flush_options = FlushOptions::default();
//         // flush_options.set_wait(true);
//         //
//         // let res = self.db.flush_cf_opt(cf_ref, &flush_options);
//         // match res {
//         //     Ok(..) => writer
//         //         .write(
//         //             format!(
//         //                 "\n{:?}:cf {}: Memtable flushed\n\n",
//         //                 chrono::offset::Utc::now(),
//         //                 self.cf_name,
//         //             )
//         //             .as_ref(),
//         //         )
//         //         .unwrap(),
//         //     Err(e) => writer
//         //         .write(
//         //             format!(
//         //                 "{:?}:flush failed Err = {}\n\n",
//         //                 chrono::offset::Utc::now(),
//         //                 e
//         //             )
//         //             .as_ref(),
//         //         )
//         //         .unwrap(),
//         // };
//     }
//
//     fn build_key(&self, op_key_suffix: Option<&str>, id: usize) -> String {
//         unsafe {
//             match op_key_suffix {
//                 Some(suf) => (String::from_utf8_unchecked(Vec::from(id.to_be_bytes()))
//                     + &*suf.to_string())
//                     .to_string(),
//                 None => String::from_utf8_unchecked(Vec::from(id.to_be_bytes())),
//             }
//         }
//     }
//     ///based on the seed we used when generating data, verify the data we got back
//     fn verify_read_res(
//         &self,
//         results: Vec<Result<Option<Vec<u8>>, Error>>,
//         batch_ids: &[usize],
//     ) -> u32 {
//         let mut num_bad_op = 0;
//
//         for (i, res) in results.into_iter().enumerate() {
//             match res {
//                 Ok(op) => match op {
//                     Some(ret_vec) => match ret_vec.len() {
//                         0 => {
//                             println!(
//                                 "Bad option : index {} id {} Len is Zero",
//                                 i,
//                                 batch_ids.get(i).unwrap()
//                             );
//                             num_bad_op += 1;
//                         }
//                         _ => {
//                             let curr_fvec: Vec<_> = ret_vec.to_owned();
//                             for (j, x) in batch_ids
//                                 .get(i)
//                                 .unwrap()
//                                 .to_be_bytes()
//                                 .into_iter()
//                                 .enumerate()
//                             {
//                                 assert_eq!(curr_fvec.as_byte_slice()[j], x)
//                             }
//                         }
//                     },
//                     None => {
//                         println!("Bad option : index {} id {} ", i, batch_ids.get(i).unwrap());
//                         num_bad_op += 1;
//                     }
//                 },
//                 Err(e) => {
//                     println!(
//                         "Error parsing header: index {} id {} {:?}",
//                         i,
//                         batch_ids.get(i).unwrap(),
//                         e
//                     );
//                     num_bad_op += 1;
//                 }
//             }
//         }
//         num_bad_op
//     }
//
//     fn report_perf<W: Write>(
//         &self,
//         load_name: &str,
//         batch_size: usize,
//         op_tot: usize,
//         op_len: usize,
//         dur_us: usize,
//         range: usize,
//         perc_done: u32,
//         writer: &mut BufWriter<W>,
//     ) -> () {
//         let time_u = chrono::offset::Utc::now();
//         if dur_us > 0 {
//             if perc_done == 100 {
//                 writer.write(format!(
//                     "\n{:?}:{}_{}_{}_batch_size_{} \n{:?}:Results=(total time {} sec, {:.2} kb/s, {:.2} op/s , \
//                 batch latency {:.2} us ) \n{:?}:Total num batches {} keys_range_size = {} progress {}% \n\n",
//                     time_u,
//                     self.name,
//                     self.cf_name,
//                     load_name,
//                     batch_size,
//                     time_u,
//                     // total time ms
//                     dur_us as f32 / MICRO_IN_SEC as f32,
//                     // kb/s
//                     (((op_tot * op_len) * MICRO_IN_SEC) as f32 / dur_us as f32) / KB as f32,
//                     // op/s
//                     (op_tot * MICRO_IN_SEC) as f32 / dur_us as f32,
//                     // batch latency us
//                     (dur_us * batch_size) as f32 / op_tot as f32,
//                     time_u,
//                     op_tot / batch_size,
//                     range,
//                     perc_done,
//                 ).as_ref()).unwrap();
//             } else {
//                 writer
//                     .write(
//                         format!(
//                             "\n{:?}: => {:.2} kb/s, {:.1} op/s , \
//                 batch latency {:.1} us, progress {}% ",
//                             time_u,
//                             // kb/s
//                             (((op_tot * op_len) * MICRO_IN_SEC) as f32 / dur_us as f32) / KB as f32,
//                             // op/s
//                             (op_tot * MICRO_IN_SEC) as f32 / dur_us as f32,
//                             // batch latency us
//                             (dur_us * batch_size) as f32 / op_tot as f32,
//                             perc_done,
//                         )
//                         .as_ref(),
//                     )
//                     .unwrap();
//             }
//         }
//     }
// }
//
// impl<W: Write> DataLoader<W> for RandomDataLoad {
//     fn run_append_seq_up_to(
//         &mut self,
//         batch_size: usize,
//         target_num: usize,
//         op_key_suffix: Option<&str>,
//         writer: &mut BufWriter<W>,
//     ) {
//         let mut dur = 0;
//         let mut op_num = 0;
//         let cf_ref = self.db.cf_handle(&*self.cf_name).unwrap();
//         let old_num_vecs = self.curr_num_vectors;
//         let mut prev_perc = 0;
//         if self.curr_num_vectors < target_num {
//             writer
//                 .write(
//                     format!(
//                         "\n\n{:?}:Inserting(upend) additional {} vectors\n\n",
//                         chrono::offset::Utc::now(),
//                         target_num - self.curr_num_vectors
//                     )
//                     .as_ref(),
//                 )
//                 .unwrap();
//         }
//         while self.curr_num_vectors < target_num {
//             let num_in_batch = min(batch_size, target_num - self.curr_num_vectors);
//             let ids_seq: Vec<usize> = (0..num_in_batch)
//                 .into_iter()
//                 .map(|x| x + self.curr_num_vectors)
//                 .collect();
//             dur += self.set_ids_data(op_key_suffix, ids_seq.as_slice(), true, false, writer);
//             let _ = self.db.iterator_cf(cf_ref, IteratorMode::Start).count();
//             //update number of vectors in db(to avoid query rocksdb for the info)
//             self.curr_num_vectors += num_in_batch;
//             op_num += 1;
//             let curr_perc = (op_num * batch_size * 100 / (target_num - old_num_vecs)) as u32;
//             if (curr_perc >= (prev_perc + PERIODIC_PRINT_INTERVAL_PERCENT)) || prev_perc == 100 {
//                 self.report_perf(
//                     "append_seq_up_to",
//                     batch_size,
//                     op_num,
//                     self.vec_len_u8,
//                     dur,
//                     target_num - old_num_vecs,
//                     curr_perc,
//                     writer,
//                 );
//                 prev_perc = curr_perc;
//             }
//         }
//         self.flush_writes(&*self.cf_name, writer);
//     }
//     fn run_append_seq(
//         &mut self,
//         batch_size: usize,
//         num: usize,
//         op_key_suffix: Option<&str>,
//         writer: &mut BufWriter<W>,
//     ) {
//         self.run_append_seq_up_to(
//             batch_size,
//             num + self.curr_num_vectors,
//             op_key_suffix,
//             writer,
//         )
//     }
//
//     fn run_overwrite_load(
//         &self,
//         batch_size: usize,
//         num_batch: usize,
//         key_range_start: usize,
//         key_range_end: usize,
//         op_key_suffix: Option<&str>,
//         writer: &mut BufWriter<W>,
//     ) {
//         let tot_num_loops = 100;
//         let mut prev_perc = 0;
//         let mut dur = 0;
//         let mut num_cycles = 0;
//         let mut ids = vec![0_usize; num_batch * batch_size];
//         ids.fill_with(|| (thread_rng().gen_range(key_range_start..key_range_end) as usize));
//         for _ in 0..tot_num_loops {
//             for batch_id in 0..num_batch {
//                 dur += self.set_ids_data(
//                     op_key_suffix,
//                     &ids[batch_id * batch_size..batch_size + (batch_id * batch_size)],
//                     true,
//                     true,
//                     writer,
//                 );
//             }
//             num_cycles += 1;
//             let curr_perc = (num_cycles * num_batch * batch_size * 100
//                 / (tot_num_loops * num_batch * batch_size)) as u32;
//             if (curr_perc >= (prev_perc + PERIODIC_PRINT_INTERVAL_PERCENT)) || prev_perc == 100 {
//                 self.report_perf(
//                     "overwrite",
//                     batch_size,
//                     num_cycles * num_batch * batch_size,
//                     self.vec_len_u8,
//                     dur,
//                     key_range_end - key_range_start,
//                     curr_perc,
//                     writer,
//                 );
//                 prev_perc = curr_perc;
//             }
//         }
//         //print_memory_usage(&db);
//         self.flush_writes(&*self.cf_name, writer);
//     }
//     fn run_seq_get(
//         &self,
//         batch_size: usize,
//         key_range_start: usize,
//         key_range_end: usize,
//         op_key_suffix: Option<&str>,
//         checksum: bool,
//         fill_cache: bool,
//         writer: &mut BufWriter<W>,
//     ) -> u32 {
//         let mut dur: usize = 0;
//         let mut vec_read = 0;
//
//         let mut ro = ReadOptions::default();
//         let verify = true;
//         let mut tot_num_bad_op = 0;
//         let mut prec_prev = 0;
//         {
//             for batch_id in key_range_start / batch_size..key_range_end / batch_size {
//                 let batch_ids: Vec<usize> =
//                     (batch_id * batch_size..(batch_id + 1) * batch_size).collect();
//                 let res = self.get_ids_data(
//                     &mut ro,
//                     op_key_suffix,
//                     batch_ids.as_slice(),
//                     checksum,
//                     fill_cache,
//                     verify,
//                 );
//                 tot_num_bad_op += res.1;
//                 dur += res.0;
//
//                 vec_read += batch_size;
//                 let prec_curr = (vec_read * 100 / (key_range_end - key_range_start)) as u32;
//                 if (prec_curr >= (prec_prev + PERIODIC_PRINT_INTERVAL_PERCENT)) || prec_curr == 100
//                 {
//                     self.report_perf(
//                         "cache_warmup_get",
//                         batch_size,
//                         vec_read,
//                         self.vec_len_u8,
//                         dur,
//                         key_range_end - key_range_start,
//                         prec_curr,
//                         writer,
//                     );
//                     prec_prev = prec_curr;
//                 }
//             }
//         }
//         tot_num_bad_op
//     }
//
//     fn run_get_load(
//         &self,
//         batch_size: usize,
//         num_loops: usize,
//         key_range_start: usize,
//         key_range_end: usize,
//         op_key_suffix: Option<&str>,
//         checksum: bool,
//         fill_cache: bool,
//         writer: &mut BufWriter<W>,
//     ) -> u32 {
//         let mut dur: usize = 0;
//         let mut vec_read = 0;
//         let mut ids = vec![0_usize; batch_size * num_loops];
//         let mut ro = ReadOptions::default();
//         let verify = true;
//         let mut tot_num_bad_op = 0;
//         let mut prec_prev = 0;
//         ids.fill_with(|| (thread_rng().gen_range(key_range_start..key_range_end) as usize));
//         {
//             for batch_id in 0..num_loops {
//                 let res = self.get_ids_data(
//                     &mut ro,
//                     op_key_suffix,
//                     &ids[batch_id * batch_size..(batch_id + 1) * batch_size],
//                     checksum,
//                     fill_cache,
//                     verify,
//                 );
//                 tot_num_bad_op += res.1;
//                 dur += res.0;
//                 vec_read += batch_size;
//                 let prec_curr = (vec_read * 100 / (num_loops * batch_size)) as u32;
//                 if (prec_curr >= (prec_prev + PERIODIC_PRINT_INTERVAL_PERCENT)) || prec_curr == 100
//                 {
//                     self.report_perf(
//                         "multi_get",
//                         batch_size,
//                         vec_read,
//                         self.vec_len_u8,
//                         dur,
//                         key_range_end - key_range_start,
//                         prec_curr,
//                         writer,
//                     );
//                     prec_prev = prec_curr;
//                 }
//             }
//         }
//         tot_num_bad_op
//     }
//     fn db_write_stats(&self, writer: &mut BufWriter<W>, flush: bool) -> () {
//         self.flush_metrics(writer);
//         if flush {
//             writer.flush().unwrap();
//         }
//     }
// }
