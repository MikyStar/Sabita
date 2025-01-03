use sabi::assets::full_grid::GRID_VALUES_1;
use sabi::core::benchmark::benchmark;
use sabi::core::cli::parse_args;
use sabi::core::grid::{print_2d_vec, Grid};
use sabi::core::validation::validate;
use sabi::utils::grid_utils::grid_values_array_to_vec;

////////////////////////////////////////

fn main() {
    showcase_solver();
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
    let generated = Grid::generate();

    println!("Generated");
    print_2d_vec(&generated.get_values());
    println!(
        "Is really valid: {}",
        validate(&generated.get_values()).is_ok()
    );
}
