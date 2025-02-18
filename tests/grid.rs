#[cfg(test)]
mod grid_tests {
    use sabita::assets::full_grid::{GRID_VALUES_1, GRID_VALUES_2};
    use sabita::core::constants::TO_BE_SOLVED;
    use sabita::core::grid::Grid;
    use sabita::utils::grid_utils::grid_values_array_to_vec;

    ////////////////////
    // Valid grid

    #[test]
    fn valid_complete_grid() {
        Grid::new(vec![
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
    }

    #[test]
    fn valid_incomplete_grid() {
        Grid::new(vec![
            vec![3, 9, 1, 2, 8, 6, 5, 7, TO_BE_SOLVED],
            vec![4, 8, 7, 3, 5, 9, 1, 2, TO_BE_SOLVED],
            vec![6, 5, 2, 7, 1, 4, 8, 3, TO_BE_SOLVED],
            vec![8, 7, 5, 4, 3, 1, 6, 9, TO_BE_SOLVED],
            vec![2, 1, 3, 9, 6, 7, 4, 8, TO_BE_SOLVED],
            vec![9, 6, 4, 5, 2, 8, 7, TO_BE_SOLVED, 3],
            vec![1, 4, 9, 6, 7, 3, TO_BE_SOLVED, 5, 8],
            vec![TO_BE_SOLVED, 3, 8, 1, 4, 2, 9, 6, 7],
            vec![7, 2, 6, TO_BE_SOLVED, 9, 5, 3, 4, 1],
        ]);
    }

    ////////////////////
    // Invalid length

    #[test]
    #[should_panic(expected = "Wrong number of lines: 0")]
    fn empty_grid() {
        Grid::new(vec![]);
    }

    #[test]
    #[should_panic(expected = "Line index 0 has a different number of columns than 9")]
    fn too_few_columns0() {
        Grid::new(vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8],
            vec![2, 3, 4, 5, 6, 7, 8, 9],
            vec![3, 4, 5, 6, 7, 8, 9, 1],
            vec![4, 5, 6, 7, 8, 9, 1, 2],
            vec![5, 6, 7, 8, 9, 1, 2, 3],
            vec![6, 7, 8, 9, 1, 2, 3, 4],
            vec![7, 8, 9, 1, 2, 3, 4, 5],
            vec![8, 9, 1, 2, 3, 4, 5, 6],
            vec![9, 1, 2, 3, 4, 5, 6, 7],
        ]);
    }

    #[test]
    #[should_panic(expected = "Line index 5 has a different number of columns than 9")]
    fn too_few_columns5() {
        Grid::new(vec![
            vec![3, 9, 1, 2, 8, 6, 5, 7, 4],
            vec![4, 8, 7, 3, 5, 9, 1, 2, 6],
            vec![6, 5, 2, 7, 1, 4, 8, 3, 9],
            vec![8, 7, 5, 4, 3, 1, 6, 9, 2],
            vec![2, 1, 3, 9, 6, 7, 4, 8, 5],
            vec![9, 6, 4, 5, 2, 8, 7, 1], // Here
            vec![1, 4, 9, 6, 7, 3, 2, 5, 8],
            vec![5, 3, 8, 1, 4, 2, 9, 6, 7],
            vec![7, 2, 6, 8, 9, 5, 3, 4, 1],
        ]);
    }

    #[test]
    #[should_panic(expected = "Wrong number of lines: 8")]
    fn too_few_lines() {
        Grid::new(vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, TO_BE_SOLVED],
            vec![2, 3, 4, 5, 6, 7, 8, 9, TO_BE_SOLVED],
            vec![3, 4, 5, 6, 7, 8, 9, 1, TO_BE_SOLVED],
            vec![4, 5, 6, 7, 8, 9, 1, 2, TO_BE_SOLVED],
            vec![5, 6, 7, 8, 9, 1, 2, 3, TO_BE_SOLVED],
            vec![6, 7, 8, 9, 1, 2, 3, 4, TO_BE_SOLVED],
            vec![7, 8, 9, 1, 2, 3, 4, 5, TO_BE_SOLVED],
            vec![8, 9, 1, 2, 3, 4, 5, 6, TO_BE_SOLVED],
        ]);
    }

    ////////////////////
    // Line checks

    #[test]
    #[should_panic(expected = "Line index 0 is not valid, duplicate value 3")]
    fn invalid_line0() {
        Grid::new(vec![
            vec![3, 3, 1, 2, 8, 6, 5, 7, 4],
            vec![4, 8, 7, 3, 5, 9, 1, 2, 6],
            vec![6, 5, 2, 7, 1, 4, 8, 3, 9],
            vec![8, 7, 5, 4, 3, 1, 6, 9, 2],
            vec![2, 1, 3, 9, 6, 7, 4, 8, 5],
            vec![9, 6, 4, 5, 2, 8, 7, 1, 3],
            vec![1, 4, 9, 6, 7, 3, 2, 5, 8],
            vec![5, 3, 8, 1, 4, 2, 9, 6, 7],
            vec![7, 2, 6, 8, 9, 5, 3, 4, 1],
        ]);
    }

    #[test]
    #[should_panic(expected = "Line index 7 is not valid, duplicate value 6")]
    fn invalid_line7() {
        Grid::new(vec![
            vec![3, 9, 1, 2, 8, 6, 5, 7, 4],
            vec![4, 8, 7, 3, 5, 9, 1, 2, TO_BE_SOLVED],
            vec![6, 5, 2, 7, 1, 4, 8, 3, 9],
            vec![8, 7, 5, 4, 3, 1, 6, 9, 2],
            vec![2, 1, 3, 9, 6, 7, 4, 8, 5],
            vec![9, 6, 4, 5, 2, 8, 7, 1, 3],
            vec![1, 4, 9, 6, 7, 3, 2, 5, 8],
            vec![5, 3, 8, 1, 4, 2, 9, 6, 6],
            vec![7, 2, 6, 8, 9, 5, 3, 4, 1],
        ]);
    }

    ////////////////////
    // Column checks

    #[test]
    #[should_panic(expected = "Column index 1 is not valid, duplicate value 5")]
    fn invalid_column1() {
        Grid::new(vec![
            vec![3, 9, 1, 2, 8, 6, 5, 7, 4],
            vec![4, 8, 7, 3, 5, 9, 1, 2, 6],
            vec![6, 5, 2, 7, 1, 4, 8, 3, 9],
            vec![8, 7, 5, 4, 3, 1, 6, 9, 2],
            vec![2, 1, 3, 9, 6, 7, 4, 8, 5],
            vec![9, 6, 4, 5, 2, 8, 7, 1, 3],
            vec![1, 4, 9, 6, 7, 3, 2, 5, 8],
            vec![5, 5, 8, 1, 4, 2, 9, 6, 7], // Here
            vec![7, 2, 6, 8, 9, 5, 3, 4, 1],
        ]);
    }

    ////////////////////
    // Region checks

    #[test]
    #[should_panic(expected = "Region index 0 is not valid, duplicate value 2")]
    fn invalid_region0() {
        Grid::new(vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, TO_BE_SOLVED],
            vec![2, 3, 4, 5, 6, 7, 8, 9, TO_BE_SOLVED],
            vec![3, 4, 5, 6, 7, 8, 9, 1, TO_BE_SOLVED],
            vec![4, 5, 6, 7, 8, 9, 1, 2, TO_BE_SOLVED],
            vec![5, 6, 7, 8, 9, 1, 2, 3, TO_BE_SOLVED],
            vec![6, 7, 8, 9, 1, 2, 3, 4, TO_BE_SOLVED],
            vec![7, 8, 9, 1, 2, 3, 4, 5, TO_BE_SOLVED],
            vec![8, 9, 1, 2, 3, 4, 5, 6, TO_BE_SOLVED],
            vec![9, 1, 2, 3, 4, 5, 6, 7, TO_BE_SOLVED],
        ]);
    }

    #[test]
    #[should_panic(expected = "Region index 8 is not valid, duplicate value 5")]
    fn invalid_region8() {
        Grid::new(vec![
            vec![3, 9, 1, 2, 8, 6, TO_BE_SOLVED, 7, 4],
            vec![4, 8, 7, 3, 5, 9, 1, 2, 6],
            vec![6, TO_BE_SOLVED, 2, 7, 1, 4, 8, 3, 9],
            vec![8, 7, 5, 4, 3, 1, 6, TO_BE_SOLVED, 2],
            vec![2, 1, 3, 9, 6, 7, 4, 8, 5],
            vec![9, 6, 4, 5, 2, 8, 7, 1, 3],
            vec![1, 4, 9, 6, 7, 3, 2, 5, 8],
            vec![TO_BE_SOLVED, 3, 8, 1, 4, 2, 5, 9, 7],
            vec![7, 5, 6, 8, 9, TO_BE_SOLVED, 3, 4, 1],
        ]);
    }

    ////////////////////
    // Remove random values

    #[test]
    #[should_panic(expected = "Can not remove that much values")]
    fn invalid_remove_random_value_full_grid() {
        let values = grid_values_array_to_vec(GRID_VALUES_2);
        let mut grid = Grid::new(values.clone());

        grid.remove_random_values(81);
    }

    #[test]
    #[should_panic(expected = "Can not remove that much values")]
    fn invalid_remove_random_value_above() {
        let values = grid_values_array_to_vec(GRID_VALUES_2);
        let mut grid = Grid::new(values.clone());

        grid.remove_random_values(90);
    }

    #[test]
    fn remove_1_random_value() {
        let values = grid_values_array_to_vec(GRID_VALUES_1);
        let mut grid = Grid::new(values.clone());

        let nb_to_remove = 1;

        grid.remove_random_values(nb_to_remove);
        let missing = grid.locate_missing_box();

        assert_eq!(missing.len(), nb_to_remove as usize);
        assert_ne!(values, grid.get_values());
    }

    #[test]
    fn remove_20_random_value() {
        let values = grid_values_array_to_vec(GRID_VALUES_2);
        let mut grid = Grid::new(values.clone());

        let nb_to_remove = 20;

        grid.remove_random_values(nb_to_remove);
        let missing = grid.locate_missing_box();

        assert_eq!(missing.len(), nb_to_remove as usize);
        assert_ne!(values, grid.get_values());
    }

    #[test]
    fn remove_80_random_value() {
        let values = grid_values_array_to_vec(GRID_VALUES_2);
        let mut grid = Grid::new(values.clone());

        let nb_to_remove = 80;

        grid.remove_random_values(nb_to_remove);
        let missing = grid.locate_missing_box();

        assert_eq!(missing.len(), nb_to_remove as usize);
        assert_ne!(values, grid.get_values());
    }
}
