use std::{
    fmt,
    sync::{
        mpsc::{channel, SyncSender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

////////////////////

pub const NB_TESTS: u8 = 50;

////////////////////

#[derive(Debug, Copy, Clone)]
pub struct BenchmarkResult {
    pub fastest: Duration,
    pub slowest: Duration,
    pub average: Duration,
    pub std_dev: Duration,
}

pub struct BenchmarkParams {
    f: Box<dyn Fn() -> Duration + Send + Sync + 'static>,
    on_thread_message: Box<dyn Fn(ThreadLifecycleMessage) + 'static>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ThreadLifecycleMsgType {
    Start,
    Stop,
}

#[derive(Debug, Copy, Clone)]
pub enum ThreadMessageType {
    Lifecycle,
    Result,
    Tick,
}

#[derive(Copy, Clone)]
pub struct FuncThreadMessage {
    pub msg_type: ThreadMessageType,
    pub msg: ThreadMessage,
    pub func: FunctionName,
}

#[derive(Copy, Clone)]
pub union ThreadMessage {
    pub lifecycle_msg: ThreadLifecycleMessage,
    pub result_msg: BenchmarkResult,
    pub tick_msg: (), // TODO remove
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
    pub msg_type: ThreadLifecycleMsgType,
    pub id: u8,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FunctionName {
    Clock,

    Generate,
    Solv10,
    Solv30,
    Solv50,
    Solv64,
}

impl fmt::Display for FunctionName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let txt = match self {
            FunctionName::Clock => "clock",
            FunctionName::Generate => "generate",
            FunctionName::Solv10 => "solv10",
            FunctionName::Solv30 => "solv30",
            FunctionName::Solv50 => "solv50",
            FunctionName::Solv64 => "solv64",
        };

        write!(f, "{txt}")
    }
}

// TODO maybe bypass this need by just using stringify!(the_func)
pub struct BenchmarkFunction {
    pub name: FunctionName,
    pub f: Box<dyn Fn() -> Duration + Send + Sync + 'static>,
}

////////////////////

pub fn benchmark_fn(args: BenchmarkParams) -> BenchmarkResult {
    let BenchmarkParams {
        f,
        on_thread_message,
    } = args;

    // TODO reuse the same channel that main function
    let (tx, rx) = channel();

    let func = Arc::new(f);

    let faster = Arc::new(Mutex::new(None));
    let slower = Arc::new(Mutex::new(None));
    let full_time = Arc::new(Mutex::new(None));

    let times: Arc<Mutex<Vec<Duration>>> = Arc::new(Mutex::new(vec![]));

    let mut thread_handles = vec![];

    for i in 0..NB_TESTS {
        let tx_clone = tx.clone();

        let func_clone = Arc::clone(&func);
        let faster_clone = Arc::clone(&faster);
        let slower_clone = Arc::clone(&slower);
        let full_time_clone = Arc::clone(&full_time);

        let times_clone = Arc::clone(&times);

        let handle = thread::spawn(move || {
            tx_clone
                .send(ThreadLifecycleMessage {
                    msg_type: ThreadLifecycleMsgType::Start,
                    id: i,
                })
                .unwrap();

            let res = func_clone();

            let mut fast = faster_clone.lock().unwrap();
            match *fast {
                None => *fast = Some(res),
                Some(val) => {
                    if val > res {
                        *fast = Some(res);
                    }
                }
            }

            let mut slow = slower_clone.lock().unwrap();
            match *slow {
                None => *slow = Some(res),
                Some(val) => {
                    if val < res {
                        *slow = Some(res);
                    }
                }
            }

            let mut full = full_time_clone.lock().unwrap();
            match *full {
                None => *full = Some(res),
                Some(val) => {
                    *full = Some(val + res);
                }
            }

            times_clone.lock().unwrap().push(res);

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

    let fastest = (faster.lock().unwrap()).unwrap();
    let slowest = (slower.lock().unwrap()).unwrap();
    let average = (full_time.lock().unwrap())
        .unwrap()
        .div_f32(NB_TESTS as f32);

    let std_dev = {
        let vals = times.lock().unwrap();
        let durations_ms: Vec<u128> = vals.iter().map(|d| d.as_nanos()).collect();

        let variance = durations_ms
            .iter()
            .map(|&ms| {
                let diff = ms as f64 - (average.as_nanos() as f64);
                diff * diff
            })
            .sum::<f64>()
            / durations_ms.len() as f64;

        let std_dev_ns = variance.sqrt();

        Duration::from_nanos(std_dev_ns as u64)
    };

    BenchmarkResult {
        fastest,
        slowest,
        average,
        std_dev,
    }
}
////////////////////

pub fn execute_benchmarks(
    sender: SyncSender<FuncThreadMessage>,
    functions: Vec<BenchmarkFunction>,
) {
    for bench_func in functions {
        let sender_clone = sender.clone();
        let BenchmarkFunction { name, f } = bench_func;

        thread::spawn(move || {
            let sender_clone_again = sender_clone.clone();

            let res = benchmark_fn(BenchmarkParams {
                f: Box::new(f),
                on_thread_message: Box::new(move |msg| {
                    sender_clone
                        .send(FuncThreadMessage {
                            func: name,
                            msg_type: ThreadMessageType::Lifecycle,
                            msg: ThreadMessage { lifecycle_msg: msg },
                        })
                        .unwrap();
                }),
            });

            sender_clone_again
                .send(FuncThreadMessage {
                    func: name,
                    msg_type: ThreadMessageType::Result,
                    msg: ThreadMessage { result_msg: res },
                })
                .unwrap();
        });
    }

    start_clock(sender);
}

fn start_clock(sender: SyncSender<FuncThreadMessage>) {
    thread::spawn(move || loop {
        let sec = Duration::from_secs(1);
        thread::sleep(sec);

        sender
            .send(FuncThreadMessage {
                func: FunctionName::Clock,
                msg_type: ThreadMessageType::Tick,
                msg: ThreadMessage { tick_msg: () },
            })
            .unwrap();
    });
}
