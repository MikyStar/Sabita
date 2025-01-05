use crate::assets::full_grid::GRID_VALUES_1;
use crate::utils::grid_utils::grid_values_array_to_vec;

use super::constants::{LENGTH_DIMENSION, MAX_NB_VALUES, TO_BE_SOLVED};
use super::grid::{location_to_region, BoxLocation, GridValues};
use super::solver::{locate_missing_box, solve};

use rand::distributions::{Distribution, Uniform};

use std::fmt;

////////////////////////////////////////

#[derive(Debug)]
pub struct GeneratingSudokuError;

impl fmt::Display for GeneratingSudokuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occured trying to generate sudoku")
    }
}

////////////////////////////////////////

/// Generates a sudoku
pub fn generate() -> Result<GridValues, GeneratingSudokuError> {
    let original: GridValues = grid_values_array_to_vec(GRID_VALUES_1);
    let mut to_return = original.clone();

    let min_round = 3;
    let mut round_counter = 0;

    while round_counter < min_round {
        let modified = remove_random_values(&to_return, 50).0;
        let missing_locations = locate_missing_box(&modified);
        to_return = solve(&modified, &missing_locations).unwrap();

        if original != to_return {
            round_counter += 1;
        }
    }

    Ok(to_return)
}

////////////////////

pub fn remove_random_values(
    values: &GridValues,
    nb_to_remove: u8,
) -> (GridValues, Vec<BoxLocation>) {
    if nb_to_remove >= MAX_NB_VALUES {
        panic!("Can not remove that much values")
    }

    let mut matrix = values.clone();

    let mut rng = rand::thread_rng();
    let pos = Uniform::from(TO_BE_SOLVED..LENGTH_DIMENSION);

    let mut loc_removed = vec![];

    while loc_removed.len() < nb_to_remove.into() {
        let line = pos.sample(&mut rng);
        let column = pos.sample(&mut rng);

        let location = BoxLocation {
            line,
            column,
            region: location_to_region(&line, &column).unwrap(),
        };

        if !loc_removed.contains(&location) {
            matrix[line as usize][column as usize] = TO_BE_SOLVED;

            loc_removed.push(location);
        }
    }

    (matrix, loc_removed)
}
