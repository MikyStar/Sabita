mod core;
use crate::core::grid::Grid;

////////////////////////////////////////

fn main() {
    let test_grid = Grid::new(vec!
        [
            vec![1, 2, 3, 4, 5, 6, 7, 8, 0],
            vec![2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![3, 4, 5, 6, 7, 8, 9, 1, 0],
            vec![4, 5, 6, 7, 8, 9, 1, 2, 0],
            vec![5, 6, 7, 8, 9, 1, 2, 3, 0],
            vec![6, 7, 8, 9, 1, 2, 3, 4, 0],
            vec![7, 8, 9, 1, 2, 3, 4, 5, 0],
            vec![8, 9, 1, 2, 3, 4, 5, 6, 0],
            vec![9, 1, 2, 3, 4, 5, 6, 7, 0],
        ]
    );

    let missing_boxes = test_grid.locate_missing_box();
    test_grid.print_grid(Some(missing_boxes))
}
