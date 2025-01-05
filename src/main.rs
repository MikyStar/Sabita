use sabita::core::benchmark::benchmark;
use sabita::core::cli::{parse_args, ArgParsed, ACTION};
use sabita::core::grid::Grid;

use std::process::exit;

////////////////////////////////////////

fn main() {
    let ArgParsed {
        action,
        path,
        nb_missing,
    } = parse_args();

    match action {
        ACTION::Solve => {
            let mut grid = Grid::from_file(path.unwrap());
            grid.solve();
            grid.print();
        }
        ACTION::Generate => {
            let grid = Grid::generate(nb_missing);
            grid.print();
            grid.dump_file(path.unwrap());
        }
        ACTION::Benchmark => {
            benchmark();
        }
        ACTION::Version => {
            version();
        }
        ACTION::HelpFull => {
            version();
            println!();
            help_generate();
            println!();
            help_solver();
            println!();
            help_benchmark();
        }
        ACTION::HelpGenerate => {
            eprintln!("Wrong args for command generate\n");
            help_generate();

            exit(-1);
        }
        ACTION::HelpSolve => {
            eprintln!("Wrong args for command solve\n");
            help_solver();

            exit(-1);
        }
    }
}

////////////////////

fn help_benchmark() {
    let name: &str = env!("CARGO_PKG_NAME");

    println!("Benchmark:");
    println!("           {name} --benchmark");
}

fn version() {
    let name: &str = env!("CARGO_PKG_NAME");
    let version: &str = env!("CARGO_PKG_VERSION");

    println!("{name} v{version}")
}

fn help_solver() {
    let name: &str = env!("CARGO_PKG_NAME");

    println!("Solver:");
    println!("         {name} s <file/to/solve>");
    println!("Example:");
    println!("         {name} s sudoku.example");
}

fn help_generate() {
    let name: &str = env!("CARGO_PKG_NAME");

    println!("Generator:");
    println!("           {name} g <file/to/create> [optional number of missing boxes]");
    println!("Example:");
    println!("           {name} g sudoku.txt");
    println!("           {name} g sudoku.txt 52");
}
