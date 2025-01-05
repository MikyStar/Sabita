use crate::assets::full_grid::GRID_VALUES_1;
use crate::utils::grid_utils::grid_values_array_to_vec;

use super::constants::{LENGTH_DIMENSION, MAX_NB_VALUES, TO_BE_SOLVED};
use super::grid::{location_to_region, BoxLocation, GridValues};
use super::solver::{locate_missing_box, solve};

use rand::distributions::{Distribution, Uniform};
use rand::Rng;

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
    let to_return = original.clone();

    let mut modified = remove_random_values(&to_return, 50).0;
    random_permutations(&mut modified);
    swap_lines(&mut modified);

    let missing_locations = locate_missing_box(&modified);
    let to_return = solve(&modified, &missing_locations).unwrap();

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

/// As swaping first and second line wont change the solution for their regions and columns
/// they can be swapped in order to create new sudokus, same for every line and columns within
/// their regions
fn swap_lines(values: &mut GridValues) {
    let first_line = values[0].clone();
    let second_line = values[1].clone();

    let fourth_line = values[3].clone();
    let fifth_line = values[4].clone();

    let seventh_line = values[6].clone();
    let eighth_line = values[7].clone();

    values[0] = second_line;
    values[1] = first_line;

    values[3] = fifth_line;
    values[4] = fourth_line;

    values[6] = eighth_line;
    values[7] = seventh_line;
}

fn random_permutations(values: &mut GridValues) {
    let mut rng = rand::thread_rng();
    let value = Uniform::from(1..LENGTH_DIMENSION + 1);

    let mut available_values = vec![];
    for i in 0..(LENGTH_DIMENSION) {
        for j in 0..(LENGTH_DIMENSION) {
            let line = i as usize;
            let col = j as usize;

            let val = values[line][col];

            if !available_values.contains(&val) {
                available_values.push(val);
            }
        }
    }

    let available_pairs = (available_values.len() / 2) as u8; // Already floored by u8 type
    let nb_permutations = rng.gen_range(2..available_pairs);

    for _ in 0..nb_permutations {
        let value_a = value.sample(&mut rng);
        let value_b = value.sample(&mut rng);

        permute_values(values, value_a, value_b);
    }
}

/// Replace every instance of value_a with value_b and vice_versa
pub fn permute_values(values: &mut GridValues, value_a: u8, value_b: u8) {
    for i in 0..(LENGTH_DIMENSION) {
        for j in 0..(LENGTH_DIMENSION) {
            let line = i as usize;
            let col = j as usize;

            let val = values[line][col];

            if val == value_a {
                values[line][col] = value_b;
            } else if val == value_b {
                values[line][col] = value_a;
            }
        }
    }
}
