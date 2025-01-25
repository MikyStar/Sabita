use super::runner::BenchmarkFunction;

////////////////////////////////////////

pub const BENCH_FILE: &str = "temp.benchmark";

////////////////////////////////////////

pub struct Config {
    pub nb_iterations: u16,
    pub functions: Vec<BenchmarkFunction>,
    pub nb_buckets_around_avg: u16,

    pub file_path: Option<String>,
}
