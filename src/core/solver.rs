use super::constants::{LENGTH_DIMENSION, TO_BE_SOLVED};
use super::grid::{print_2d_vec, GridValues};
use super::validation::{is_column_valid, is_line_valid, is_region_valid};
use crate::core::grid::location_to_region;

use itertools::izip;
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

pub fn reduce_solutions(grid_values: &GridValues, missing_boxes: &Vec<Vec<u8>>) {
    let mut grid_copy = grid_values.clone();
    let mut missing_boxes_copy = missing_boxes.clone();

    let sols = get_solutions_complexity_sorted(grid_values, missing_boxes);
    let regions = sols
        .iter()
        .map(|sol| location_to_region(sol.0).unwrap())
        .collect::<Vec<u8>>();

    for (index, ((line_col, solutions), region)) in izip!(&sols, &regions).enumerate() {
        println!("{}:{:?}({}) -> {:?}", index, line_col, region, solutions);

        let mut involved_forward_boxes_indices = vec![];

        if index + 1 < sols.len() {
            for other_box_index in (index + 1)..sols.len() {
                let same_line = sols[other_box_index].0[0] == line_col[0];
                let same_col = sols[other_box_index].0[1] == line_col[1];
                let same_region = regions[other_box_index] == *region;

                if same_line | same_col | same_region {
                    involved_forward_boxes_indices.push(other_box_index);
                }
            }
        }

        println!("\t{:?}", involved_forward_boxes_indices)
    }

    // TODO check in line col region which values doesn't intersect

    // remove item in array
    // grid_copy[missing[0] as usize][missing[1] as usize] = solutions[0];
    //
    // let index = missing_boxes_copy
    //     .iter()
    //     .position(|x| *x == *missing)
    //     .unwrap();
    // missing_boxes_copy.remove(index);
}

/// Returns boxes and solutions ordered by their number of possibilities (asc)
pub fn get_solutions_complexity_sorted<'a>(
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
