use sabita::core::{
    bench::benchmark,
    cli::{parse_args, ArgParsed, ACTION},
    constants::{PKG_NAME, PKG_VERSION},
    grid::Grid,
};

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

            exit(1);
        }
        ACTION::HelpSolve => {
            eprintln!("Wrong args for command solve\n");
            help_solver();

            exit(1);
        }
    }
}

////////////////////

fn help_benchmark() {
    println!("Benchmark:");
    println!("           {PKG_NAME} --benchmark");
}

fn version() {
    println!("{PKG_NAME} v{PKG_VERSION}")
}

fn help_solver() {
    println!("Solver:");
    println!("         {PKG_NAME} s <file/to/solve>");
    println!("Example:");
    println!("         {PKG_NAME} s sudoku.example");
}

fn help_generate() {
    println!("Generator:");
    println!("           {PKG_NAME} g <file/to/create> [optional number of missing boxes]");
    println!("Example:");
    println!("           {PKG_NAME} g sudoku.txt");
    println!("           {PKG_NAME} g sudoku.txt 52");
}
