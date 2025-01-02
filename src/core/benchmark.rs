use crate::core::generator;

use super::grid::Grid;

use std::fmt;
use std::time::{Duration, Instant};

use humanize_duration::prelude::DurationExt;
use humanize_duration::Truncate;

////////////////////////////////////////

pub const NB_TESTS: u8 = 3;

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

        write!(
            f,
            "Randomizing {NB_TESTS} solutions\n\n{solver}\n\n{generator}"
        )
    }
}

////////////////////

#[derive(Debug)]
pub struct BenchmarkResult {
    faster: Duration,
    slower: Duration,
    average: Duration,
}

impl fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let avg = &self.average.human(Truncate::Nano);
        let fast = &self.faster.human(Truncate::Nano);
        let slow = &self.slower.human(Truncate::Nano);

        write!(f, "Average: {avg}\nFaster: {fast}\nSlower: {slow}")
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

pub fn benchmark() -> FullBenchmark {
    FullBenchmark {
        solver: benchmark_solvers(),
        generator: benchmark_generators(),
    }
}

fn benchmark_one_generate() -> Duration {
    let start = Instant::now();
    let _grid = Grid::generate();
    start.elapsed()
}

fn benchmark_generators() -> BenchmarkResult {
    let mut faster: Option<Duration> = None;
    let mut slower: Option<Duration> = None;
    let mut full_time: Option<Duration> = None;

    for _i in 0..NB_TESTS {
        let res = benchmark_one_generate();

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
        faster: faster.unwrap(),
        slower: slower.unwrap(),
        average: full_time.unwrap().div_f32(NB_TESTS as f32),
    }
}

fn benchmark_solvers() -> BenchmarkSolver {
    BenchmarkSolver {
        missing_ten: benchmark_multiple_solver(10),
        missing_thirty: benchmark_multiple_solver(30),
        missing_fifty: benchmark_multiple_solver(50),
        missing_sixty_four: benchmark_multiple_solver(64),
    }
}

fn benchmark_multiple_solver(nb_to_remove: u8) -> BenchmarkResult {
    let mut faster: Option<Duration> = None;
    let mut slower: Option<Duration> = None;
    let mut full_time: Option<Duration> = None;

    for _i in 0..NB_TESTS {
        let res = benchmark_one_solver(nb_to_remove);

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
        faster: faster.unwrap(),
        slower: slower.unwrap(),
        average: full_time.unwrap().div_f32(NB_TESTS as f32),
    }
}

fn benchmark_one_solver(nb_to_remove: u8) -> Duration {
    let mut grid = Grid::generate();
    grid.remove_random_values(nb_to_remove);

    let start = Instant::now();
    grid.solve().unwrap();

    start.elapsed()
}
