use super::constants::{LENGTH_DIMENSION, TO_BE_SOLVED};
use super::grid::{location_to_region, BoxLocation, GridValues};
use super::solver::solve;

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::{fmt, usize};

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
    let mut matrix: GridValues = vec![vec![0; 9]; 9];

    // Creating a matrix with random first and last line

    let mut first_line = init_values();
    first_line.shuffle(&mut thread_rng());

    let mut last_line = init_values();
    last_line.shuffle(&mut thread_rng());

    for i in 0..LENGTH_DIMENSION {
        let index = i as usize;

        // Making last line compliant
        while first_line[index] == last_line[index] {
            let to_move = last_line.remove(i.into());
            last_line.push(to_move);
        }

        matrix[0][index] = first_line[index];
        matrix[(LENGTH_DIMENSION - 1) as usize][index] = last_line[index];
    }

    // Solve the remaining boxes

    let mut missing_locations = vec![];

    for line_index in 1..(LENGTH_DIMENSION - 1) {
        for col_index in 0..LENGTH_DIMENSION {
            let loc = BoxLocation {
                line: line_index,
                column: col_index,
                region: location_to_region(&line_index, &col_index).unwrap(),
            };
            missing_locations.push(loc);
        }
    }

    match solve(&matrix, &missing_locations) {
        Ok(solved) => Ok(solved),
        Err(_) => Err(GeneratingSudokuError),
    }
}

////////////////////

fn init_values() -> Vec<u8> {
    ((TO_BE_SOLVED + 1)..(LENGTH_DIMENSION + 1)).collect()
}
