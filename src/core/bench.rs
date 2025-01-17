use super::benchmark::{
    message_handler::handle_messages,
    runner::{
        execute_benchmarks, BenchmarkFunction, BenchmarkResult, FuncThreadMessage, FunctionName,
    },
};

use super::{
    constants::{PKG_NAME, PKG_VERSION},
    grid::Grid,
};

use std::{
    fmt,
    sync::mpsc::sync_channel,
    time::{Duration, Instant},
};

////////////////////////////////////////

pub const NB_TESTS: u8 = 50;

////////////////////

#[derive(Debug)]
pub struct FullBenchmark {
    solver: BenchmarkSolver,
    generator: BenchmarkResult,
}

impl fmt::Display for FullBenchmark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let solver = format!("----- Solver -----\n\n{}", self.solver);
        let generator = format!("----- Generator -----\n\n{}", self.generator);

        write!(f, "{solver}\n\n{generator}")
    }
}

////////////////////

#[derive(Debug)]
pub struct BenchmarkSolver {
    missing_ten: BenchmarkResult,
    missing_thirty: BenchmarkResult,
    missing_fifty: BenchmarkResult,
    missing_sixty_four: BenchmarkResult,
}

impl fmt::Display for BenchmarkSolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let section_10 = format!("** Missing 10 **\n{}", self.missing_ten);
        let section_30 = format!("** Missing 30 **\n{}", self.missing_thirty);
        let section_50 = format!("** Missing 50 **\n{}", self.missing_fifty);
        let section_64 = format!("** Missing 64 **\n{}", self.missing_sixty_four);

        write!(
            f,
            "{section_10}\n\n{section_30}\n\n{section_50}\n\n{section_64}"
        )
    }
}

////////////////////////////////////////

pub fn benchmark() {
    let start = Instant::now();

    println!("----------------------------------------\n");
    println!("Benchmarking {PKG_NAME}@v{PKG_VERSION} with {NB_TESTS} iterations\n");

    let (tx, rx) = sync_channel::<FuncThreadMessage>(1);

    let to_bench: Vec<BenchmarkFunction> = vec![
        BenchmarkFunction {
            name: FunctionName::Generate,
            f: Box::new(benchmark_one_generate),
        },
        BenchmarkFunction {
            name: FunctionName::Solv10,
            f: Box::new(solv_10),
        },
        BenchmarkFunction {
            name: FunctionName::Solv30,
            f: Box::new(solv_30),
        },
        BenchmarkFunction {
            name: FunctionName::Solv50,
            f: Box::new(solv_50),
        },
        BenchmarkFunction {
            name: FunctionName::Solv64,
            f: Box::new(solv_64),
        },
    ];

    let f_names = to_bench.iter().map(|f| f.name).collect();

    execute_benchmarks(tx, to_bench);
    handle_messages(rx, f_names, start);
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
