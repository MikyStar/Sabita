use super::constants::{LENGTH_DIMENSION, TO_BE_SOLVED};
use super::grid::{BoxLocation, GridValues};
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

#[derive(Clone)]
pub struct SortedSolution<'a> {
    location: &'a BoxLocation,
    solutions: Vec<u8>,
}

    line: &'a u8,
    column: &'a u8,
    region: u8,
    solutions: Vec<u8>,
}

////////////////////////////////////////

pub fn reduce_solutions(grid_values: &GridValues, missing_boxes: &Vec<BoxLocation>) {
    let mut grid_copy = grid_values.clone();
    let mut missing_boxes_copy = missing_boxes.clone();

    let sols = get_solutions_complexity_sorted(grid_values, missing_boxes);

    // TODO externalize as it's too usefull
    for (index, sol) in sols.clone().into_iter().enumerate() {
        let SortedSolution {
            location:
                BoxLocation {
                    line,
                    column,
                    region,
                },
            solutions,
        } = sol;

        println!(
            "{} = [{}:{}]({}) -> {:?}",
            index, line, column, region, solutions
        );

        let mut involved_forward_boxes_indices = vec![];

        if index + 1 < sols.len() {
            for other_box_index in (index + 1)..sols.len() {
                let other_box_location = sols[other_box_index].location;

                let same_line = other_box_location.line == *line;
                let same_col = other_box_location.column == *column;
                let same_region = other_box_location.region == *region;

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

/// Returns boxes, regions and solutions ordered by their number of possibilities (asc)
pub fn get_solutions_complexity_sorted<'a>(
    grid_values: &GridValues,
    missing_boxes: &'a Vec<BoxLocation>,
) -> Vec<SortedSolution<'a>> {
    let mut solutions = vec![];

    for missing in missing_boxes.iter() {
        let solution = get_box_solutions(&grid_values, &missing).unwrap();
        solutions.push(solution);
    }

    let mut sorted_indices = (0..solutions.len()).collect::<Vec<usize>>();
    sorted_indices.sort_by_key(|&i| solutions[i].len());

    let mut locs_regions_solutions = vec![];

    for index in sorted_indices {
        let combo = SortedSolution {
            location: &missing_boxes[index],
            solutions: solutions[index].clone(),
        };

        locs_regions_solutions.push(combo);
    }

    locs_regions_solutions
}

pub fn get_box_solutions(
    grid_values: &GridValues,
    location: &BoxLocation,
) -> Result<Vec<u8>, BoxSolutionNotFound> {
    let mut answers: Vec<u8> = vec![];

    for possibility in (TO_BE_SOLVED + 1)..(LENGTH_DIMENSION + 1) {
        let mut grid_to_test = grid_values.clone();
        grid_to_test[location.line as usize][location.column as usize] = possibility;

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
