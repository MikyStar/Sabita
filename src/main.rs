use sabi::core::constants::TO_BE_SOLVED;
use sabi::core::grid::{print_grid, Grid};
use sabi::core::solver::get_box_solutions;

////////////////////////////////////////

fn main() {
    let grid = Grid::new(vec![
        vec![TO_BE_SOLVED, 9, 1, 2, 8, 6, 5, 7, 4], // Should be 3
        vec![4, 8, 7, 3, 5, 9, 1, 2, 6],
        vec![6, 5, 2, 7, 1, 4, 8, 3, 9],
        vec![8, 7, 5, 4, 3, 1, 6, 9, 2],
        vec![2, 1, 3, 9, 6, 7, 4, 8, 5],
        vec![9, 6, 4, 5, 2, 8, 7, 1, 3],
        vec![1, 4, 9, 6, 7, 3, 2, 5, 8],
        vec![5, 3, 8, 1, 4, 2, 9, 6, 7],
        vec![7, 2, 6, 8, 9, 5, 3, 4, TO_BE_SOLVED], // Should be 1
    ]);

    println!("Full grid");
    print_grid(&grid.get_values());

    println!("\nMissing boxes positions");
    let missing_boxes = grid.locate_missing_box();
    print_grid(&missing_boxes);

    println!("\nSolutions");
    for location in missing_boxes {
        let solutions = get_box_solutions(&grid.get_values(), &location);

        match solutions {
            Ok(solutions) => println!("{:?} -> {:?}", location, solutions),
            Err(err) => panic!("{}", err),
        }
    }
}
