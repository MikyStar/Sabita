use super::{
    benchmark::{
        benchmark::{benchmark as lib_bench, Config},
        file::FilePolicy,
        runner::BenchmarkFunction,
    },
    grid::Grid,
};

use std::time::{Duration, Instant};

////////////////////////////////////////

const BENCH_FILE: &str = "temp.benchmark";

////////////////////////////////////////

pub fn benchmark() {
    lib_bench(Config {
        file_path: Some(BENCH_FILE.to_string()),
        default_file_policy: Some(FilePolicy::Rewrite),
        nb_buckets_around_avg: 3,
        nb_iterations: 50,
        functions: vec![
            BenchmarkFunction {
                name: "generate".to_string(),
                f: Box::new(benchmark_one_generate),
            },
            // BenchmarkFunction {
            //     name: "solv10".to_string(),
            //     f: Box::new(solv_10),
            // },
            // BenchmarkFunction {
            //     name: "solv30".to_string(),
            //     f: Box::new(solv_30),
            // },
            // BenchmarkFunction {
            //     name: "solv50".to_string(),
            //     f: Box::new(solv_50),
            // },
            // BenchmarkFunction {
            //     name: "solv64",
            //     f: Box::new(solv_64),
            // },
        ],
    });
}

////////////////////

fn benchmark_one_generate() -> Duration {
    let start = Instant::now();
    Grid::generate(None);
    start.elapsed()
}

fn benchmark_one_solver(nb_to_remove: u8) -> Duration {
    let mut grid = Grid::generate(None);
    grid.remove_random_values(nb_to_remove);

    let start = Instant::now();
    grid.solve();
    start.elapsed()
}

fn solv_10() -> Duration {
    benchmark_one_solver(10)
}

fn solv_30() -> Duration {
    benchmark_one_solver(30)
}

fn solv_50() -> Duration {
    benchmark_one_solver(50)
}

fn solv_64() -> Duration {
    benchmark_one_solver(64)
}
