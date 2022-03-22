use std::io::BufWriter;
use std::io::Write;
/// Trait that defines an indexer. Indexers can be incremental or batch oriented.
/// An indexer is assumed to deal with fixes size vectors
pub trait DataLoader<W: Write>: Send + Sync {
    /// run insert\put sequntially on on the given key range. each batch write request has
    /// batch_size vectors. this is used for sequential prefill
    fn run_append_seq_up_to(
        &mut self,
        batch_size: usize,
        target_num: usize,
        op_key_suffix: Option<&str>,
        writer: &mut BufWriter<W>,
    ) -> ();
    /// run insert\put sequntially on on the given key range. each batch write request has
    /// batch_size vectors. this is used for sequential prefill
    fn run_append_seq(
        &mut self,
        batch_size: usize,
        num: usize,
        op_key_suffix: Option<&str>,
        writer: &mut BufWriter<W>,
    ) -> ();
    /// run update randomly on on the given key range. each batch write request has
    /// batch_size vectors. run in loop for num_batch times.
    fn run_overwrite_load(
        &mut self,
        batch_size: usize,
        num_batch: usize,
        key_range_start: usize,
        key_range_end: usize,
        op_key_suffix: Option<&str>,
        writer: &mut BufWriter<W>,
    ) -> ();

    /// run multi-get sequential load on the given key range. each multi-get request has
    /// batch_size vectors. run in loop for num_loops times. this can be used to measure
    /// sequential access as well as to prehit cache(like a simple iterator)
    fn run_seq_get(
        &mut self,
        batch_size: usize,
        key_range_start: usize,
        key_range_end: usize,
        op_key_suffix: Option<&str>,
        checksum: bool,
        fill_cache: bool,
        writer: &mut BufWriter<W>,
    ) -> u32;
    /// run multi-get random load on the given key range. each multi-get request has batch_size vectors.
    /// run in loop for num_loops times
    fn run_get_load(
        &mut self,
        batch_size: usize,
        num_loops: usize,
        key_range_start: usize,
        key_range_end: usize,
        op_key_suffix: Option<&str>,
        checksum: bool,
        fill_cache: bool,
        writer: &mut BufWriter<W>,
    ) -> u32;
    fn db_write_stats(&self, writer: &mut BufWriter<W>, flush: bool) -> ();
}
