mod core;
use crate::core::grid::Grid;
use crate::core::grid::TO_BE_SOLVED;

////////////////////////////////////////

fn main() {
    let test_grid = Grid::new(vec!
        [
            vec![1, 2, 3, 4, 5, 6, 7, 8, TO_BE_SOLVED],
            vec![2, 3, 4, 5, 6, 7, 8, 9, TO_BE_SOLVED],
            vec![3, 4, 5, 6, 7, 8, 9, 1, TO_BE_SOLVED],
            vec![4, 5, 6, 7, 8, 9, 1, 2, TO_BE_SOLVED],
            vec![5, 6, 7, 8, 9, 1, 2, 3, TO_BE_SOLVED],
            vec![6, 7, 8, 9, 1, 2, 3, 4, TO_BE_SOLVED],
            vec![7, 8, 9, 1, 2, 3, 4, 5, TO_BE_SOLVED],
            vec![8, 9, 1, 2, 3, 4, 5, 6, TO_BE_SOLVED],
            vec![9, 1, 2, 3, 4, 5, 6, 7, TO_BE_SOLVED],
        ]
    );

    let missing_boxes = test_grid.locate_missing_box();
    test_grid.print_grid(Some(missing_boxes))
}
