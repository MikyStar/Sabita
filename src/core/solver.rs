use super::constants::{LENGTH_DIMENSION, TO_BE_SOLVED};
use super::grid::GridValues;
use super::validation::{is_column_valid, is_line_valid, is_region_valid};
use std::fmt;

////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct BoxSolutionNotFound;

impl fmt::Display for BoxSolutionNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Solution of the box coul'nt be found") // TODO add location
    }
}

////////////////////////////////////////

pub fn get_box_solutions(
    grid_values: &GridValues,
    location: &Vec<u8>,
) -> Result<Vec<u8>, BoxSolutionNotFound> {
    let mut answers: Vec<u8> = vec![];

    for possibility in (TO_BE_SOLVED + 1)..(LENGTH_DIMENSION + 1) {
        let mut grid_to_test = grid_values.clone();
        grid_to_test[location[0] as usize][location[1] as usize] = possibility;

        let mut was_an_error_found = false;

        for index in 0..LENGTH_DIMENSION.into() {
            let line_valid = is_line_valid(&grid_to_test, &index).0;
            let column_valid = is_column_valid(&grid_to_test, &index).0;
            let region_valid = is_region_valid(&grid_to_test, &index).0;

            if !line_valid | !column_valid | !region_valid {
                was_an_error_found = true;
                break;
            }
        }

        if !was_an_error_found {
            answers.push(possibility)
        }
    }

    if answers.is_empty() {
        Err(BoxSolutionNotFound)
    } else {
        Ok(answers)
    }
}
