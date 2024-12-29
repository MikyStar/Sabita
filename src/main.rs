use sabi::core::constants::TO_BE_SOLVED as tbd;
use sabi::core::grid::Grid;
use sabi::core::solver::solve;

////////////////////////////////////////

fn main() {
    let grid = Grid::new(vec![
        vec![tbd, 9, 1, tbd, 8, 6, 5, 7, tbd], // Should be 3, 2, 4
        vec![tbd, 8, 7, tbd, 5, 9, 1, 2, 6],   // Should be 4, 3
        vec![6, tbd, 2, 7, 1, 4, 8, 3, 9],     // Should be 5
        vec![8, 7, 5, 4, 3, 1, 6, 9, 2],
        vec![tbd, 1, 3, 9, 6, 7, 4, 8, 5], // Should be 2
        vec![9, 6, 4, 5, 2, 8, 7, 1, 3],
        vec![1, 4, 9, 6, 7, 3, 2, 5, 8],
        vec![5, 3, 8, 1, 4, 2, 9, 6, 7],
        vec![7, 2, 6, 8, 9, 5, 3, 4, tbd], // Should be 1
    ]);

    // let grid = Grid::new(vec![
    //     vec![tbd, 9, 2, tbd, 5, tbd, tbd, 8, 6],
    //     vec![6, tbd, tbd, 8, 2, 7, 1, 5, 9],
    //     vec![tbd, tbd, 1, 3, 9, 6, 7, 2, 4],
    //     vec![2, 6, 5, 9, 7, 3, 8, 4, 1],
    //     vec![4, 8, 9, 5, 6, 1, 2, 7, 3],
    //     vec![tbd, 1, 7, 4, 8, 2, 9, 6, 5],
    //     vec![1, 3, 6, 7, 4, 8, 5, 9, 2],
    //     vec![9, 7, 4, 2, 1, 5, 6, 3, 8],
    //     vec![tbd, 2, 8, 6, 3, 9, 4, 1, 7],
    // ]);

    println!("Full grid");
    grid.print();

    println!("\nSolutions");
    let missing_boxes = grid.locate_missing_box();

    solve(&grid.get_values(), &missing_boxes);
}
