#[cfg(test)]
mod solver_e2e {
    use sabi::assets::full_grid::GRID_VALUES_1;
    use sabi::core::grid::Grid;
    use sabi::core::validation::validate;
    use sabi::utils::grid_utils::grid_values_array_to_vec;

    fn solve_test(nb_missing: u8) {
        let original = grid_values_array_to_vec(GRID_VALUES_1);

        let mut to_solve = Grid::new(original.clone());
        to_solve.remove_random_values(nb_missing);

        let res = to_solve.solve().unwrap();

        assert!(validate(&res).is_ok())
    }

    //////////

    #[test]
    fn can_solve_1_missing() {
        solve_test(1);
    }

    #[test]
    fn can_solve_10_missing() {
        solve_test(10);
    }

    #[test]
    fn can_solve_30_missing() {
        solve_test(30);
    }

    #[test]
    fn can_solve_80_missing() {
        solve_test(80);
    }
}
