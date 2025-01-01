use sabi::assets::full_grid::GRID_VALUES_1;
use sabi::core::grid::{print_2d_vec, Grid};
use sabi::core::validation::validate;
use sabi::utils::grid_utils::grid_values_array_to_vec;

////////////////////////////////////////

fn main() {
    let original = grid_values_array_to_vec(GRID_VALUES_1);
    let mut to_solve = Grid::new(original.clone());
    to_solve.remove_random_values(50);

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
