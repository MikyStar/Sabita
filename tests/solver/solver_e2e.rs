#[cfg(test)]
mod solver_e2e {
    use sabi::assets::full_grid::GRID_VALUES_1;
    use sabi::core::constants::TO_BE_SOLVED;
    use sabi::core::grid::Grid;
    use sabi::core::validation::validate;
    use sabi::utils::grid_utils::grid_values_array_to_vec;

    fn solve_test(nb_missing: u8) {
        let original = grid_values_array_to_vec(GRID_VALUES_1);

        let mut to_solve = Grid::new(original.clone());
        to_solve.remove_random_values(nb_missing);

        let res = to_solve.solve().unwrap();

        assert!(validate(&res).is_ok(), "Grid isn't valid");

        let mut unsolved_box_found = false;
        for line in res.into_iter() {
            for val in line.into_iter() {
                if val == TO_BE_SOLVED {
                    unsolved_box_found = true;
                }
            }
        }
        assert!(
            unsolved_box_found == false,
            "At least one unsolved box remaining"
        );
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
