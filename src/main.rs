use sabi::core::constants::TO_BE_SOLVED;
use sabi::core::grid::{location_to_region, Grid};
use sabi::core::solver::sort_solutions_complexity;

////////////////////////////////////////

fn main() {
    let grid = Grid::new(vec![
        vec![TO_BE_SOLVED, 9, 1, TO_BE_SOLVED, 8, 6, 5, 7, TO_BE_SOLVED], // Should be 3, 2, 4
        vec![TO_BE_SOLVED, 8, 7, TO_BE_SOLVED, 5, 9, 1, 2, 6],            // Should be 4, 3
        vec![6, TO_BE_SOLVED, 2, 7, 1, 4, 8, 3, 9],                       // Should be 5
        vec![8, 7, 5, 4, 3, 1, 6, 9, 2],
        vec![TO_BE_SOLVED, 1, 3, 9, 6, 7, 4, 8, 5], // Should be 2
        vec![9, 6, 4, 5, 2, 8, 7, 1, 3],
        vec![1, 4, 9, 6, 7, 3, 2, 5, 8],
        vec![5, 3, 8, 1, 4, 2, 9, 6, 7],
        vec![7, 2, 6, 8, 9, 5, 3, 4, TO_BE_SOLVED], // Should be 1
    ]);

    println!("Full grid");
    grid.print();

    println!("\nSolutions");
    let missing_boxes = grid.locate_missing_box();
    let solutions = sort_solutions_complexity(&grid.get_values(), &missing_boxes);

    for (location, solutions) in solutions {
        println!("{:?} -> {:?}", location, solutions)
    }

    let region = location_to_region(&vec![8, 8]).unwrap();
    println!("reg {}", region)
}
