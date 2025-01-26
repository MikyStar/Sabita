use std::{
    fmt,
    mem::ManuallyDrop,
    sync::{
        mpsc::{channel, SyncSender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

////////////////////

pub const NB_TESTS: u8 = 50;
pub const CLOCK_FN_NAME: &str = "clock";

////////////////////

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub fastest: Duration,
    pub slowest: Duration,
    pub average: Duration,
    pub std_dev: Duration,
    pub times: Vec<Duration>,
}

pub struct BenchmarkParams {
    f: Box<dyn Fn() -> Duration + Send + Sync + 'static>,
    on_thread_message: Box<dyn Fn(ThreadLifecycleMessage) + 'static>,
}

#[derive(Debug, PartialEq)]
pub enum ThreadLifecycleMsgType {
    Start,
    Stop,
}

#[derive(Debug)]
pub enum ThreadMessageType {
    Lifecycle,
    Result,
    Tick,
}

pub struct FuncThreadMessage {
    pub msg_type: ThreadMessageType,
    pub msg: ThreadMessage,
    pub func: String,
}

pub union ThreadMessage {
    pub lifecycle_msg: ManuallyDrop<ThreadLifecycleMessage>,
    pub result_msg: ManuallyDrop<BenchmarkResult>,
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

#[derive(Debug)]
pub struct ThreadLifecycleMessage {
    pub msg_type: ThreadLifecycleMsgType,
    pub id: u8,
}

pub struct BenchmarkFunction {
    pub name: String,
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

            tx_clone
                .send(ThreadLifecycleMessage {
                    msg_type: ThreadLifecycleMsgType::Stop,
                    id: i,
                })
                .unwrap();

            /////

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

    let times_vals = times.lock().unwrap();

    let std_dev = {
        let durations_ms: Vec<u128> = times_vals.iter().map(|d| d.as_nanos()).collect();

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
        times: times_vals.to_vec(),
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
        let n = name.clone();

        thread::spawn(move || {
            let sender_clone_again = sender_clone.clone();

            let res = benchmark_fn(BenchmarkParams {
                f: Box::new(f),
                on_thread_message: Box::new(move |msg| {
                    sender_clone
                        .send(FuncThreadMessage {
                            func: name.clone(),
                            msg_type: ThreadMessageType::Lifecycle,
                            msg: ThreadMessage {
                                lifecycle_msg: ManuallyDrop::new(msg),
                            },
                        })
                        .unwrap();
                }),
            });

            sender_clone_again
                .send(FuncThreadMessage {
                    func: n,
                    msg_type: ThreadMessageType::Result,
                    msg: ThreadMessage {
                        result_msg: ManuallyDrop::new(res),
                    },
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
                func: CLOCK_FN_NAME.to_string(),
                msg_type: ThreadMessageType::Tick,
                msg: ThreadMessage { tick_msg: () },
            })
            .unwrap();
    });
}
