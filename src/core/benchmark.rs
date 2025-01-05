use super::grid::Grid;

use std::fmt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use humanize_duration::prelude::DurationExt;
use humanize_duration::Truncate;

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
        generator: benchmark_fn(benchmark_one_generate),
    };

    println!("{results}");
}

////////////////////

fn benchmark_one_generate() -> Duration {
    let start = Instant::now();
    Grid::generate(None);
    start.elapsed()
}

fn benchmark_solvers() -> BenchmarkSolver {
    let miss_10_thread = thread::spawn(|| benchmark_fn(solv_10));
    let miss_30_thread = thread::spawn(|| benchmark_fn(solv_30));
    let miss_50_thread = thread::spawn(|| benchmark_fn(solv_50));
    let miss_64_thread = thread::spawn(|| benchmark_fn(solv_64));

    BenchmarkSolver {
        missing_ten: miss_10_thread.join().unwrap(),
        missing_thirty: miss_30_thread.join().unwrap(),
        missing_fifty: miss_50_thread.join().unwrap(),
        missing_sixty_four: miss_64_thread.join().unwrap(),
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
    let mut grid = Grid::generate(None);
    grid.remove_random_values(nb_to_remove);

    let start = Instant::now();
    grid.solve();
    start.elapsed()
}

////////////////////

fn benchmark_fn(f: impl Fn() -> Duration + Send + Sync + 'static) -> BenchmarkResult {
    let func = Arc::new(f);

    let faster = Arc::new(Mutex::new(None));
    let slower = Arc::new(Mutex::new(None));
    let full_time = Arc::new(Mutex::new(None));

    let mut thread_handles = vec![];

    for _i in 0..NB_TESTS {
        let func_clone = Arc::clone(&func);
        let faster_clone = Arc::clone(&faster);
        let slower_clone = Arc::clone(&slower);
        let full_time_clone = Arc::clone(&full_time);

        let handle = thread::spawn(move || {
            let res = func_clone();

            {
                let mut fast = faster_clone.lock().unwrap();
                match *fast {
                    None => *fast = Some(res),
                    Some(val) => {
                        if val > res {
                            *fast = Some(res);
                        }
                    }
                }
            }

            {
                let mut slow = slower_clone.lock().unwrap();
                match *slow {
                    None => *slow = Some(res),
                    Some(val) => {
                        if val < res {
                            *slow = Some(res);
                        }
                    }
                }
            }

            {
                let mut full = full_time_clone.lock().unwrap();
                match *full {
                    None => *full = Some(res),
                    Some(val) => {
                        *full = Some(val + res);
                    }
                }
            }
        });

        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    let fastest = (*faster.lock().unwrap()).unwrap();
    let slowest = (slower.lock().unwrap()).unwrap();
    let average = (*full_time.lock().unwrap())
        .unwrap()
        .div_f32(NB_TESTS as f32);

    BenchmarkResult {
        fastest,
        slowest,
        average,
    }
}
