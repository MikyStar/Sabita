use super::constants::{LENGTH_DIMENSION, TO_BE_SOLVED};
use super::grid::{print_2d_vec, GridValues};
use super::validation::{is_column_valid, is_line_valid, is_region_valid};

use std::fmt;

////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct BoxSolutionNotFound;

impl fmt::Display for BoxSolutionNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Solution of the box could'nt be found")
    }
}

////////////////////////////////////////

/// Returns boxes and solutions ordered by their number of possibilities (asc)
pub fn sort_solutions_complexity<'a>(
    grid_values: &GridValues,
    missing_boxes: &'a Vec<Vec<u8>>,
) -> Vec<(&'a Vec<u8>, Vec<u8>)> {
    let mut solutions = vec![];

    for missing in missing_boxes.iter() {
        let solution = get_box_solutions(&grid_values, &missing).unwrap();
        solutions.push(solution);
    }

    let mut sorted_indices = (0..solutions.len()).collect::<Vec<usize>>();
    sorted_indices.sort_by_key(|&i| solutions[i].len());

    let mut locations_and_solutions = vec![];

    for index in sorted_indices {
        let combo = (&missing_boxes[index], solutions[index].clone());
        locations_and_solutions.push(combo);
    }

    locations_and_solutions
}

pub fn get_box_solutions(
    grid_values: &GridValues,
    location: &[u8],
) -> Result<Vec<u8>, BoxSolutionNotFound> {
    let mut answers: Vec<u8> = vec![];

    for possibility in (TO_BE_SOLVED + 1)..(LENGTH_DIMENSION + 1) {
        let mut grid_to_test = grid_values.clone();
        grid_to_test[location[0] as usize][location[1] as usize] = possibility;

        let mut was_an_error_found = false;

        for index in 0..LENGTH_DIMENSION {
            let is_line_valid = is_line_valid(&grid_to_test, &index).0;
            let is_column_valid = is_column_valid(&grid_to_test, &index).0;
            let is_region_valid = is_region_valid(&grid_to_test, &index).0;

            if !is_line_valid | !is_column_valid | !is_region_valid {
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
