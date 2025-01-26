use super::{
    benchmark::{
        config::BENCH_FILE,
        console_ui::queue_msg,
        file::{handle_file, write},
        message_handler::handle_messages,
        runner::{execute_benchmarks, BenchmarkFunction, FuncThreadMessage, NB_TESTS},
    },
    constants::{PKG_NAME, PKG_VERSION},
    grid::Grid,
};

use std::{
    sync::mpsc::sync_channel,
    time::{Duration, Instant},
};

////////////////////////////////////////

pub fn benchmark() {
    handle_file(BENCH_FILE.to_string());

    let start = Instant::now();

    let txt = format!("Benchmarking {PKG_NAME}@v{PKG_VERSION} with {NB_TESTS} iterations\n");

    queue_msg(txt.clone());
    write(BENCH_FILE.to_string(), vec![txt]);

    let (tx, rx) = sync_channel::<FuncThreadMessage>(1);

    // TODO add std_vec_sort https://github.com/Voultapher/driftsort

    let to_bench: Vec<BenchmarkFunction> = vec![
        BenchmarkFunction {
            name: "generate".to_string(),
            f: Box::new(benchmark_one_generate),
        },
        BenchmarkFunction {
            name: "solv10".to_string(),
            f: Box::new(solv_10),
        },
        BenchmarkFunction {
            name: "solv30".to_string(),
            f: Box::new(solv_30),
        },
        BenchmarkFunction {
            name: "solv50".to_string(),
            f: Box::new(solv_50),
        },
        // BenchmarkFunction {
        //     name: "solv64",
        //     f: Box::new(solv_64),
        // },
    ];

    let f_names = to_bench.iter().map(|f| f.name.clone()).collect();

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
