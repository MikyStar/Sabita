use sabi::assets::full_grid::GRID_VALUES_1;
use sabi::core::benchmark::benchmark;
use sabi::core::cli::{parse_args, ArgParsed, ACTION};
use sabi::core::grid::{print_2d_vec, Grid};
use sabi::core::validation::validate;
use sabi::utils::grid_utils::grid_values_array_to_vec;

////////////////////////////////////////

fn main() {
    // showcase_solver();
    // println!();
    // showcase_generator();
    // println!();
    // benchmark();
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
        }
        ACTION::HelpGenerate => {
            help_generate();
        }
        ACTION::HelpSolve => {
            help_solver();
        }
    }
}

////////////////////

fn showcase_solver() {
    println!("----- Solver -----\n");
    let value_array = GRID_VALUES_1;
    let original = grid_values_array_to_vec(value_array);
    let mut to_solve = Grid::from_array(value_array);
    to_solve.remove_random_values(70);

    println!("Full grid");
    to_solve.print();

    let missing = to_solve.locate_missing_box();

    println!("\nSolved");
    to_solve.solve();
    to_solve.print();

    println!("\nSolved boxes {}", missing.len());

    println!("Is same as start: {}", to_solve.get_values() == original);
    println!(
        "Is really valid: {}",
        validate(&to_solve.get_values()).is_ok()
    );
}

fn showcase_generator() {
    println!("----- Generator -----\n");
    let generated = Grid::generate(None);

    println!("Generated");
    print_2d_vec(&generated.get_values());
    println!(
        "Is really valid: {}",
        validate(&generated.get_values()).is_ok()
    );
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
