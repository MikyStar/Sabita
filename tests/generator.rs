#[cfg(test)]
mod generator_tests {
    use sabi::assets::full_grid::GRID_VALUES_1;
    use sabi::core::constants::LENGTH_DIMENSION;
    use sabi::core::generator::generate;
    use sabi::core::grid::GridValues;
    use sabi::utils::grid_utils::grid_values_array_to_vec;

    ////////////////////

    #[test]
    fn generation() {
        let original: GridValues = grid_values_array_to_vec(GRID_VALUES_1);

        let generated = generate().unwrap();

        assert_ne!(original, generated, "Sudokus should be different");
        assert_eq!(
            generated.len(),
            LENGTH_DIMENSION as usize,
            "Bad number of lines"
        );

        for (index, line) in generated.iter().enumerate() {
            assert_eq!(
                line.len(),
                LENGTH_DIMENSION as usize,
                "Bad number of columns in line {index}"
            );
        }
    }
}
