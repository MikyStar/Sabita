use sabi::core::constants::TO_BE_SOLVED as tbd;
use sabi::core::grid::{print_2d_vec, Grid};

////////////////////////////////////////

fn main() {
    let original = Grid::new(vec![
        vec![3, 9, 1, 2, 8, 6, 5, 7, 4],
        vec![4, 8, 7, 3, 5, 9, 1, 2, 6],
        vec![6, 5, 2, 7, 1, 4, 8, 3, 9],
        vec![8, 7, 5, 4, 3, 1, 6, 9, 2],
        vec![2, 1, 3, 9, 6, 7, 4, 8, 5],
        vec![9, 6, 4, 5, 2, 8, 7, 1, 3],
        vec![1, 4, 9, 6, 7, 3, 2, 5, 8],
        vec![5, 3, 8, 1, 4, 2, 9, 6, 7],
        vec![7, 2, 6, 8, 9, 5, 3, 4, 1],
    ]);

    let to_solve = Grid::new(vec![
        vec![0, 0, 0, 0, 0, 6, 0, 0, 4],
        vec![0, 0, 0, 3, 0, 0, 1, 2, 6],
        vec![0, 5, 0, 7, 0, 4, 0, 3, 9],
        vec![0, 7, 5, 0, 0, 1, 6, 9, 2],
        vec![2, 0, 3, 0, 0, 7, 0, 0, 0],
        vec![9, 6, 4, 0, 2, 0, 7, 1, 0],
        vec![0, 0, 0, 6, tbd, 3, 2, 5, 0], // This element breaks the solving
        vec![5, 0, 0, 1, 4, 2, 0, 0, 0],
        vec![0, 2, 6, 0, 0, 0, 0, 0, 0],
    ]);

    println!("Full grid");
    to_solve.print();

    let missing = to_solve.locate_missing_box();

    println!("\nSolved");
    let res = to_solve.solve().unwrap();

    print_2d_vec(&res);
    println!("\nSolved boxes {}", missing.len());

    println!("Is same as start: {}", res == original.get_values());
}
