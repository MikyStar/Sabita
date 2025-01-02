use super::grid::Grid;

use std::fmt;
use std::time::{Duration, Instant};

use humanize_duration::prelude::DurationExt;
use humanize_duration::Truncate;

////////////////////////////////////////

pub const NB_TESTS: u8 = 10;

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
pub struct BenchmarkResult {
    fastest: Duration,
    slowest: Duration,
    average: Duration,
}

impl fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let avg = &self.average.human(Truncate::Nano);
        let fast = &self.fastest.human(Truncate::Nano);
        let slow = &self.slowest.human(Truncate::Nano);

        write!(f, "Average: {avg}\nFastest: {fast}\nSlowest: {slow}")
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
    let version: &str = env!("CARGO_PKG_VERSION");

    println!("----------------------------------------\n");
    println!("Benchmarking v{version} with {NB_TESTS} iterations\n");

    let results = FullBenchmark {
        solver: benchmark_solvers(),
        generator: benchmark_fn(&benchmark_one_generate),
    };

    println!("{results}");
}

fn benchmark_one_generate() -> Duration {
    let start = Instant::now();
    let _grid = Grid::generate();
    start.elapsed()
}

fn benchmark_solvers() -> BenchmarkSolver {
    let missing_ten = benchmark_fn(&solv_10);
    let missing_thirty = benchmark_fn(&solv_30);
    let missing_fifty = benchmark_fn(&solv_50);
    let missing_sixty_four = benchmark_fn(&solv_64);

    BenchmarkSolver {
        missing_ten,
        missing_thirty,
        missing_fifty,
        missing_sixty_four,
    }
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

fn benchmark_one_solver(nb_to_remove: u8) -> Duration {
    let mut grid = Grid::generate();
    grid.remove_random_values(nb_to_remove);

    let start = Instant::now();
    grid.solve().unwrap();

    start.elapsed()
}

////////////////////

fn benchmark_fn(f: &dyn Fn() -> Duration) -> BenchmarkResult {
    let mut faster: Option<Duration> = None;
    let mut slower: Option<Duration> = None;
    let mut full_time: Option<Duration> = None;

    for _i in 0..NB_TESTS {
        let res = f();

        match faster {
            None => faster = Some(res),
            Some(val) => {
                if val > res {
                    faster = Some(res);
                }
            }
        }

        match slower {
            None => slower = Some(res),
            Some(val) => {
                if val < res {
                    slower = Some(res);
                }
            }
        }

        match full_time {
            None => full_time = Some(res),
            Some(val) => {
                full_time = Some(val + res);
            }
        }
    }

    BenchmarkResult {
        fastest: faster.unwrap(),
        slowest: slower.unwrap(),
        average: full_time.unwrap().div_f32(NB_TESTS as f32),
    }
}
