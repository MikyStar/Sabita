use sabi::assets::full_grid::GRID_VALUES_1;
use sabi::core::benchmark::benchmark;
use sabi::core::cli::parse_args;
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
    let args = parse_args();
    println!("{args}");
}

////////////////////

fn showcase_solver() {
    println!("----- Solver -----\n");
    let original = grid_values_array_to_vec(GRID_VALUES_1);
    let mut to_solve = Grid::new(original.clone());
    to_solve.remove_random_values(70);

    println!("Full grid");
    to_solve.print();

    let missing = to_solve.locate_missing_box();

    println!("\nSolved");
    let res = to_solve.solve().unwrap();

    print_2d_vec(&res);
    println!("\nSolved boxes {}", missing.len());

    println!("Is same as start: {}", res == original);
    println!("Is really valid: {}", validate(&res).is_ok());
}

fn showcase_generator() {
    println!("----- Generator -----\n");
    let generated = Grid::generate();

    println!("Generated");
    print_2d_vec(&generated.get_values());
    println!(
        "Is really valid: {}",
        validate(&generated.get_values()).is_ok()
    );
}
