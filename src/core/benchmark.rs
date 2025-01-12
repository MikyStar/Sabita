use super::constants::{PKG_NAME, PKG_VERSION};
use super::grid::Grid;

use mpsc::SyncSender;
use std::fmt;
use std::sync::{mpsc, Arc, Mutex};
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

#[derive(Debug, Copy, Clone)]
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

////////////////////

pub struct BenchmarkParams {
    f: Box<dyn Fn() -> Duration + Send + Sync + 'static>,
    on_thread_message: Box<dyn Fn(ThreadLifecycleMessage) + 'static>,
}

#[derive(Debug, Copy, Clone)]
pub enum ThreadLifecycleMsgType {
    Start,
    Stop,
}

impl fmt::Display for ThreadLifecycleMsgType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let txt = match self {
            ThreadLifecycleMsgType::Start => "start",
            ThreadLifecycleMsgType::Stop => "stop",
        };

        write!(f, "{txt}")
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ThreadLifecycleMessage {
    msg_type: ThreadLifecycleMsgType,
    id: u8,
}

#[derive(Debug, Copy, Clone)]
pub enum FunctionName {
    Generate,
    Solv10,
    Solv30,
    Solv50,
    Solv64,
}

impl fmt::Display for FunctionName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let txt = match self {
            FunctionName::Generate => "generate",
            FunctionName::Solv10 => "solv10",
            FunctionName::Solv30 => "solv30",
            FunctionName::Solv50 => "solv50",
            FunctionName::Solv64 => "solv64",
        };

        write!(f, "{txt}")
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ThreadMessageType {
    Lifecycle,
    Result,
}

#[derive(Copy, Clone)]
pub struct FuncThreadMessage {
    msg_type: ThreadMessageType,
    msg: ThreadMessage,
    func: FunctionName,
}

#[derive(Copy, Clone)]
pub union ThreadMessage {
    lifecycle_msg: ThreadLifecycleMessage,
    result_msg: BenchmarkResult,
}

////////////////////////////////////////

pub fn benchmark() {
    println!("----------------------------------------\n");
    println!("Benchmarking {PKG_NAME}@v{PKG_VERSION} with {NB_TESTS} iterations\n");

    let (tx, rx) = mpsc::sync_channel::<FuncThreadMessage>(2);

    // let results = FullBenchmark {
    //     solver: benchmark_solvers(tx.clone()),
    //     generator: (|| {
    //         let clone = tx.clone();
    //
    //         benchmark_fn(BenchmarkParams {
    //             f: Box::new(benchmark_one_generate),
    //             on_thread_message: Box::new(move |msg: ThreadMessage| {
    //                 clone
    //                     .send(BenchThreadMessage {
    //                         thread_msg: msg,
    //                         func: FunctionName::Generate,
    //                     })
    //                     .unwrap();
    //             }),
    //         })
    //     })(),
    // };

    benchmark_solvers2(tx);

    for received in rx {
        let FuncThreadMessage {
            func,
            msg_type,
            msg,
        } = received;

        unsafe {
            match msg_type {
                ThreadMessageType::Lifecycle => {
                    let parsed_msg: ThreadLifecycleMessage = msg.lifecycle_msg;

                    println!(
                        "===== LIFE {}: {} {}",
                        func, parsed_msg.msg_type, parsed_msg.id
                    );
                }
                ThreadMessageType::Result => {
                    let parsed_msg: BenchmarkResult = msg.result_msg;

                    println!("===== RES {}: {}", func, parsed_msg);
                }
            }
        }
    }

    // println!("{results}");
}

////////////////////

fn benchmark_one_generate() -> Duration {
    let start = Instant::now();
    Grid::generate(None);
    start.elapsed()
}

fn benchmark_solvers2(sender: SyncSender<FuncThreadMessage>) {
    let s10 = sender.clone();
    thread::spawn(move || {
        let func = FunctionName::Solv10;
        let s10_copy = s10.clone();

        let res = benchmark_fn(BenchmarkParams {
            f: Box::new(solv_10),
            on_thread_message: Box::new(move |msg| {
                s10.send(FuncThreadMessage {
                    func,
                    msg_type: ThreadMessageType::Lifecycle,
                    msg: ThreadMessage { lifecycle_msg: msg },
                })
                .unwrap();
            }),
        });

        s10_copy
            .send(FuncThreadMessage {
                func,
                msg_type: ThreadMessageType::Result,
                msg: ThreadMessage { result_msg: res },
            })
            .unwrap();
    });

    let s30 = sender.clone();
    thread::spawn(move || {
        let func = FunctionName::Solv30;
        let s30_copy = s30.clone();

        let res = benchmark_fn(BenchmarkParams {
            f: Box::new(solv_30),
            on_thread_message: Box::new(move |msg| {
                s30.send(FuncThreadMessage {
                    func,
                    msg_type: ThreadMessageType::Lifecycle,
                    msg: ThreadMessage { lifecycle_msg: msg },
                })
                .unwrap();
            }),
        });

        s30_copy
            .send(FuncThreadMessage {
                func,
                msg_type: ThreadMessageType::Result,
                msg: ThreadMessage { result_msg: res },
            })
            .unwrap();
    });

    let s50 = sender.clone();
    thread::spawn(move || {
        let func = FunctionName::Solv50;
        let s50_copy = s50.clone();

        let res = benchmark_fn(BenchmarkParams {
            f: Box::new(solv_10),
            on_thread_message: Box::new(move |msg| {
                s50.send(FuncThreadMessage {
                    func,
                    msg_type: ThreadMessageType::Lifecycle,
                    msg: ThreadMessage { lifecycle_msg: msg },
                })
                .unwrap();
            }),
        });

        s50_copy
            .send(FuncThreadMessage {
                func,
                msg_type: ThreadMessageType::Result,
                msg: ThreadMessage { result_msg: res },
            })
            .unwrap();
    });

    let s64 = sender.clone();
    thread::spawn(move || {
        let func = FunctionName::Solv64;
        let s64_copy = s64.clone();

        let res = benchmark_fn(BenchmarkParams {
            f: Box::new(solv_64),
            on_thread_message: Box::new(move |msg| {
                s64.send(FuncThreadMessage {
                    func,
                    msg_type: ThreadMessageType::Lifecycle,
                    msg: ThreadMessage { lifecycle_msg: msg },
                })
                .unwrap();
            }),
        });

        s64_copy
            .send(FuncThreadMessage {
                func,
                msg_type: ThreadMessageType::Result,
                msg: ThreadMessage { result_msg: res },
            })
            .unwrap();
    });
}

fn benchmark_solvers(sender: SyncSender<FuncThreadMessage>) -> BenchmarkSolver {
    fn on_msg(msg: ThreadLifecycleMessage) {
        println!("Solver thread {}: {}", msg.id, msg.msg_type);
    }

    let miss_10_thread = thread::spawn(move || {
        benchmark_fn(BenchmarkParams {
            f: Box::new(solv_10),
            on_thread_message: Box::new(on_msg),
            // on_thread_message: Box::new(move |msg| {
            //     let clone = sender.clone();
            //
            //     clone
            //         .send(FuncThreadMessage {
            //             msg,
            //             func: FunctionName::Solv10,
            //         })
            //         .unwrap();
            // }),
        })
    });
    let miss_30_thread = thread::spawn(|| {
        benchmark_fn(BenchmarkParams {
            f: Box::new(solv_30),
            on_thread_message: Box::new(on_msg),
        })
    });
    let miss_50_thread = thread::spawn(|| {
        benchmark_fn(BenchmarkParams {
            f: Box::new(solv_50),
            on_thread_message: Box::new(on_msg),
        })
    });
    let miss_64_thread = thread::spawn(|| {
        benchmark_fn(BenchmarkParams {
            f: Box::new(solv_64),
            on_thread_message: Box::new(on_msg),
        })
    });

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

fn benchmark_fn(args: BenchmarkParams) -> BenchmarkResult {
    let BenchmarkParams {
        f,
        on_thread_message,
    } = args;

    let (tx, rx) = mpsc::channel();

    let func = Arc::new(f);

    let faster = Arc::new(Mutex::new(None));
    let slower = Arc::new(Mutex::new(None));
    let full_time = Arc::new(Mutex::new(None));

    let mut thread_handles = vec![];

    for i in 0..NB_TESTS {
        let tx_clone = tx.clone();

        let _func_clone = Arc::clone(&func);
        let faster_clone = Arc::clone(&faster);
        let slower_clone = Arc::clone(&slower);
        let full_time_clone = Arc::clone(&full_time);

        let handle = thread::spawn(move || {
            tx_clone
                .send(ThreadLifecycleMessage {
                    msg_type: ThreadLifecycleMsgType::Start,
                    id: i,
                })
                .unwrap();

            // let res = func_clone();
            let res = Duration::new(5, 0);
            thread::sleep(res);

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

            tx_clone
                .send(ThreadLifecycleMessage {
                    msg_type: ThreadLifecycleMsgType::Stop,
                    id: i,
                })
                .unwrap();
        });

        thread_handles.push(handle);
    }
    // Drop the original sender to avoid deadlock (main thread won't produce messages)
    drop(tx);

    for received in rx {
        on_thread_message(received);
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
