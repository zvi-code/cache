// use cache::performance::data_loader::DataLoader;
// use cache::performance::store_perf_helper::RandomDataLoad;
// // use pinecone_db::storage::store_config::StoreMode;
// use std::cmp::min;
// // use rayon::iter::{IntoParallelIterator, ParallelIterator};
// use cache_proj::performance::data_loader::DataLoader;
// use std::fs::OpenOptions;
// use std::io::{BufWriter, Write};
// use std::os::raw::c_char;
// use structopt::StructOpt;
// #[allow(unused)]
// use sys_info::{
//     boottime, cpu_num, cpu_speed, disk_info, hostname, linux_os_release, loadavg, mem_info,
//     os_release, os_type, proc_total,
// };
//
// #[derive(Debug, StructOpt)]
// struct InputOpt {
//     #[structopt(long = "dimension", default_value = "768")]
//     dimension: usize,
//     #[structopt(long = "n_loops", default_value = "50000")]
//     n_loops: usize,
//     #[structopt(long = "w_batch_size", default_value = "10000")]
//     w_batch_size: usize,
//     #[structopt(long = "ow_loops", default_value = "0")]
//     ow_loops: usize,
//     #[structopt(long = "threads", default_value = "1")]
//     threads: usize,
//     #[structopt(long = "num_vectors", default_value = "1000000")]
//     num_vectors: usize,
//     #[structopt(long = "max_results", default_value = "10000")]
//     max_results: usize,
//     #[structopt(long = "use_mmap", default_value = "0")]
//     use_mmap: usize,
//     #[structopt(long = "use_direct", default_value = "0")]
//     use_direct: usize,
//     #[structopt(long = "checksum", parse(try_from_str), default_value = "false")]
//     checksum: bool,
//     #[structopt(short, long = "full_run", parse(try_from_str), default_value = "true")]
//     full_run: bool,
//     #[structopt(long = "fill_cache", parse(try_from_str), default_value = "false")]
//     fill_cache: bool,
//     #[structopt(long = "cache_warmup", parse(try_from_str), default_value = "false")]
//     cache_warmup: bool,
//     #[structopt(long = "active_percent", default_value = "100")]
//     active_percent: usize,
//     #[structopt(long = "cache_hit_ratio", default_value = "0")]
//     cache_hit_ratio: usize,
//     #[structopt(long = "cache_size_mb", default_value = "0")]
//     cache_size_mb: usize,
//     #[structopt(long = "rocksdb_stats", parse(try_from_str), default_value = "true")]
//     rocksdb_stats: bool,
//     //todo add option to specify ingest and modify load. these loads can run with quota based
//     // limiter possibly in different thread
// }
// pub fn print_node_info<W: Write>(writer: &mut BufWriter<W>) {
//     writer
//         .write(
//             format!(
//                 "\n{:?}:=========== Start Node info =========== \n",
//                 chrono::offset::Utc::now()
//             )
//             .as_ref(),
//         )
//         .unwrap();
//     writer
//         .write(
//             format!(
//                 "\n{:?}:os: {} {}",
//                 chrono::offset::Utc::now(),
//                 os_type().unwrap(),
//                 os_release().unwrap()
//             )
//             .as_ref(),
//         )
//         .unwrap();
//     writer
//         .write(
//             format!(
//                 "\n{:?}:cpu: {} cores, {} MHz",
//                 chrono::offset::Utc::now(),
//                 cpu_num().unwrap(),
//                 cpu_speed().unwrap()
//             )
//             .as_ref(),
//         )
//         .unwrap();
//     writer
//         .write(
//             format!(
//                 "\n{:?}:proc total: {}",
//                 chrono::offset::Utc::now(),
//                 proc_total().unwrap()
//             )
//             .as_ref(),
//         )
//         .unwrap();
//     writer
//         .write(
//             format!(
//                 "\n{:?}:hostname: {}",
//                 chrono::offset::Utc::now(),
//                 hostname().unwrap()
//             )
//             .as_ref(),
//         )
//         .unwrap();
//     #[cfg(target_os = "linux")]
//     writer
//         .write(
//             format!(
//                 "\n{:?}:/etc/os-release: {:?}",
//                 chrono::offset::Utc::now(),
//                 linux_os_release().unwrap()
//             )
//             .as_ref(),
//         )
//         .unwrap();
//     #[cfg(not(target_os = "windows"))]
//     {
//         let t = boottime().unwrap();
//         writer
//             .write(
//                 format!(
//                     "\n{:?}:boottime {} sec, {} usec",
//                     chrono::offset::Utc::now(),
//                     t.tv_sec,
//                     t.tv_usec
//                 )
//                 .as_ref(),
//             )
//             .unwrap();
//     }
// }
// pub fn print_memory_usage<W: Write>(phase_name: String, writer: &mut BufWriter<W>) {
//     writer
//         .write(
//             format!(
//                 "\n{:?}:Resource Usage info:{}",
//                 chrono::offset::Utc::now(),
//                 phase_name
//             )
//             .as_ref(),
//         )
//         .unwrap();
//     let load_avg = loadavg().unwrap();
//     writer
//         .write(
//             format!(
//                 "\n{:?}:===> cpu_usage:    one minute {}%, five minutes {}%, fifteen minutes {}%",
//                 chrono::offset::Utc::now(),
//                 load_avg.one,
//                 load_avg.five,
//                 load_avg.fifteen,
//             )
//             .as_ref(),
//         )
//         .unwrap();
//     let mem = mem_info().unwrap();
//     writer.write(format!(
//         "\n{:?}:===> mem: total {} MB, free {} MB, avail {} MB, buffers {} MB, cached {} MB, swap: total {} MB, free {} MB",
//         chrono::offset::Utc::now(),
//         mem.total / 1024 as u64,
//         mem.free / 1024 as u64,
//         mem.avail / 1024 as u64,
//         mem.buffers / 1024 as u64,
//         mem.cached / 1024 as u64,
//         mem.swap_total / 1024 as u64,
//         mem.swap_free / 1024 as u64
//     ).as_ref()).unwrap();
//
//     #[cfg(not(target_os = "solaris"))]
//     {
//         let disk = disk_info().unwrap();
//         writer
//             .write(
//                 format!(
//                     "\n{:?}:===> disk:         total {} MB, free {} MB\n\n",
//                     chrono::offset::Utc::now(),
//                     disk.total / 1024 as u64,
//                     disk.free / 1024 as u64
//                 )
//                 .as_ref(),
//             )
//             .unwrap();
//     }
// }
// fn main() -> Result<(), std::io::Error> {
//     let opt: InputOpt = InputOpt::from_args();
//
//     let configs_to_run = if opt.full_run {
//         vec![
//             BenchPatams {
//                 mode: StoreMode::Hybrid,
//                 config_id: 1,
//                 sstbl: "Block".to_string(),
//                 n_vecs: 5 * opt.num_vectors,
//                 max_res: opt.max_results,
//                 dim: opt.dimension,
//                 use_mmap: 0,
//                 use_direct: 3,
//                 cache_size_mb: 16 as u32,
//                 num_threads: 1,
//             },
//             BenchPatams {
//                 mode: StoreMode::Hybrid,
//                 config_id: 3,
//                 sstbl: "Block".to_string(),
//                 n_vecs: 5 * opt.num_vectors,
//                 max_res: opt.max_results,
//                 dim: opt.dimension,
//                 use_mmap: 0,
//                 use_direct: 3,
//                 cache_size_mb: 256 as u32,
//                 num_threads: 1,
//             },
//             BenchPatams {
//                 mode: StoreMode::Mem,
//                 config_id: 2,
//                 sstbl: "Cuckoo".to_string(),
//                 n_vecs: opt.num_vectors,
//                 max_res: opt.max_results,
//                 dim: opt.dimension,
//                 use_mmap: 1,
//                 use_direct: 0,
//                 cache_size_mb: 32 as u32,
//                 num_threads: 1,
//             },
//             BenchPatams {
//                 mode: StoreMode::Mem,
//                 config_id: 4,
//                 sstbl: "Plain".to_string(),
//                 n_vecs: opt.num_vectors,
//                 max_res: opt.max_results,
//                 dim: opt.dimension,
//                 use_mmap: 1,
//                 use_direct: 0,
//                 cache_size_mb: 32 as u32,
//                 num_threads: 1,
//             },
//         ]
//     } else {
//         vec![
//             BenchPatams {
//                 mode: StoreMode::Hybrid,
//                 config_id: 1,
//                 sstbl: "Block".to_string(),
//                 n_vecs: opt.num_vectors,
//                 max_res: opt.max_results,
//                 dim: opt.dimension,
//                 use_mmap: opt.use_mmap,
//                 use_direct: opt.use_direct,
//                 cache_size_mb: opt.cache_size_mb as u32,
//                 num_threads: opt.threads,
//             },
//             BenchPatams {
//                 mode: StoreMode::Mem,
//                 config_id: 2,
//                 sstbl: "Cuckoo".to_string(),
//                 n_vecs: opt.num_vectors,
//                 max_res: opt.max_results,
//                 dim: opt.dimension,
//                 use_mmap: opt.use_mmap,
//                 use_direct: opt.use_direct,
//                 cache_size_mb: opt.cache_size_mb as u32,
//                 num_threads: opt.threads,
//             },
//         ]
//     };
//     rayon::ThreadPoolBuilder::new()
//         .num_threads(opt.threads)
//         .build_global()
//         .unwrap();
//     bench_rocks_db_all(&opt, configs_to_run);
//     Ok(())
// }
// #[allow(unused)]
// struct BenchPatams {
//     mode: StoreMode,
//     config_id: u64,
//     sstbl: String,
//     n_vecs: usize,
//     max_res: usize,
//     dim: usize,
//     use_mmap: usize,
//     use_direct: usize,
//     cache_size_mb: u32,
//     num_threads: usize,
// }
// fn bench_rocks_db_all(user_op: &InputOpt, configs_to_run: Vec<BenchPatams>) {
//     for bnanch_params in configs_to_run {
//         let f = OpenOptions::new()
//             .append(true)
//             .create(true)
//             .open(format!(
//                 "/tmp/store_perf/{}.{}.{}.{}.{}.{}",
//                 bnanch_params.mode.to_string(),
//                 bnanch_params.config_id,
//                 bnanch_params.sstbl,
//                 bnanch_params.use_mmap,
//                 bnanch_params.use_direct,
//                 bnanch_params.cache_size_mb
//             ))
//             .expect(&*format!(
//                 "Unable to open file path /tmp/store_perf/{}.{}.{}.{}.{}.{}",
//                 bnanch_params.mode.to_string(),
//                 bnanch_params.config_id,
//                 bnanch_params.sstbl,
//                 bnanch_params.use_mmap,
//                 bnanch_params.use_direct,
//                 bnanch_params.cache_size_mb
//             ));
//         //f.rewind().unwrap();
//
//         let mut writer = BufWriter::new(f);
//         print_node_info(&mut writer);
//         print_memory_usage("initial".to_string(), &mut writer);
//         let mut loader = RandomDataLoad::new(
//             bnanch_params.mode,
//             bnanch_params.config_id,
//             Some(bnanch_params.sstbl.clone()),
//             bnanch_params.n_vecs,
//             bnanch_params.dim,
//             bnanch_params.use_mmap as u8,
//             bnanch_params.use_direct as u8,
//             bnanch_params.cache_size_mb as u32,
//             &mut writer,
//         );
//         let load_range = loader.get_range_for_cache_hit_expect(user_op.cache_hit_ratio);
//         for num_vec_f in 0..5 {
//             //run the performance tests
//             bench_rocksdb_with_type(
//                 &mut loader,
//                 user_op,
//                 &bnanch_params,
//                 (bnanch_params.mode.to_string() + &*"_".to_string()).clone(),
//                 min(
//                     load_range,
//                     ((num_vec_f + 1) * bnanch_params.n_vecs / 5) as u32,
//                 ),
//                 (num_vec_f + 1) * bnanch_params.n_vecs / 5,
//                 &mut writer,
//             );
//
//             print_memory_usage(
//                 format!(
//                     "num_vecs={}, vec_size={}",
//                     (num_vec_f + 1) * bnanch_params.n_vecs / 5,
//                     bnanch_params.dim * 4,
//                 )
//                 .to_string(),
//                 &mut writer,
//             );
//         }
//         loader.db_write_stats(&mut writer, true);
//         loader.db_gracefull_shutdown(&mut writer);
//     }
// }
// fn bench_rocksdb_with_type<W: Write + std::marker::Sync + std::marker::Send>(
//     ctx: &mut dyn DataLoader<W>,
//     user_op: &InputOpt,
//     param_of_bench: &BenchPatams,
//     name: String,
//     load_range: u32,
//     num_vectors: usize,
//     writer: &mut BufWriter<W>,
// ) {
//     let active_size = ((load_range * user_op.active_percent as u32) / 100) as usize;
//     ctx.db_write_stats(writer, false);
//
//     writer
//         .write(
//             format!(
//                 "\n\n{:?}:{} Init Num Vectors {} ,active range 0..{}, vectors size {} \n\n",
//                 chrono::offset::Utc::now(),
//                 name,
//                 num_vectors,
//                 active_size,
//                 param_of_bench.dim * 4,
//             )
//             .as_ref(),
//         )
//         .unwrap();
//
//     ctx.run_append_seq_up_to(user_op.w_batch_size, num_vectors, None, writer);
//     ctx.db_write_stats(writer, false);
//     if user_op.ow_loops != 0 {
//         writer
//             .write(
//                 format!(
//                     "\n\n{:?}:Overwrite_load: batch size {} overwrite loops {} active size {}..{}\n\n",
//                     chrono::offset::Utc::now(),user_op.w_batch_size, user_op.ow_loops, 0, active_size
//                 )
//                 .as_ref(),
//             )
//             .unwrap();
//         ctx.run_overwrite_load(
//             user_op.w_batch_size,
//             user_op.ow_loops,
//             0,
//             active_size,
//             None,
//             writer,
//         );
//         ctx.db_write_stats(writer, false);
//     }
//     if user_op.cache_warmup {
//         writer.write(
//             format!(
//                 "\n\n{:?}:{} Running Warmap (Sequential scan) batch size {} range {}..{} fill_cache=TRUE checksum={}\n\n",
//                 chrono::offset::Utc::now(),name, 10_000, 0, active_size, user_op.checksum.to_string()
//             ).as_ref()).unwrap();
//         ctx.run_seq_get(10_000, 0, active_size, None, user_op.checksum, true, writer);
//         ctx.db_write_stats(writer, false);
//     }
//     let mut num_bad: u32 = 0;
//     for max_res_f in 0..5 {
//         let max_res_now = (max_res_f + 1) * param_of_bench.max_res / 5;
//         if max_res_now > num_vectors {
//             break;
//         }
//         writer
//             .write(
//                 format!(
//                     "\n\n{:?}:{} Start Load: max_res={} active_range=({}..{})  \n\n",
//                     chrono::offset::Utc::now(),
//                     name,
//                     max_res_now,
//                     0,
//                     active_size
//                 )
//                 .as_ref(),
//             )
//             .unwrap();
//         num_bad += ctx.run_get_load(
//             max_res_now,
//             user_op.n_loops,
//             0,
//             active_size,
//             None,
//             user_op.checksum,
//             user_op.fill_cache,
//             writer,
//         );
//         ctx.db_write_stats(writer, false);
//     }
//     // let num_bad: u32 = (0..param_of_bench.num_threads)
//     //     .into_par_iter()
//     //     .map(|_| {
//     //         ctx.run_get_load(
//     //             param_of_bench.max_res / param_of_bench.num_threads,
//     //             user_op.n_loops,
//     //             0,
//     //             active_size,
//     //             None,
//     //             user_op.checksum,
//     //             user_op.fill_cache,
//     //             writer.,
//     //         )
//     //     })
//     //     .sum();
//     writer
//         .write(
//             format!(
//                 "\n\n{:?}:{} Finished!! Time  ({} Failed)\n\n",
//                 chrono::offset::Utc::now(),
//                 name,
//                 num_bad
//             )
//             .as_ref(),
//         )
//         .unwrap();
// }
