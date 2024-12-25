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

#[derive(Debug, Clone)]
pub struct SortedSolution<'a> {
    location: &'a BoxLocation,
    solutions: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct InvolvedSolutions<'a> {
    location: &'a BoxLocation,
    solutions: &'a Vec<u8>,
    involved_forward: Vec<&'a BoxLocation>,
}

////////////////////////////////////////

pub fn reduce_solutions(grid_values: &GridValues, missing_boxes: &Vec<BoxLocation>) {
    let mut grid_copy = grid_values.clone();
    let mut missing_boxes_copy = missing_boxes.clone();

    let sols = get_solutions_complexity_sorted(grid_values, missing_boxes);
    let with_involved = get_involved_solutions(&sols);

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

/// For each box location, find which other boxes will be involved in a forward way, which means
/// that if the vector passed says a location A comes before an other location B, A will mention B but
/// not the other way arround
pub fn get_involved_solutions<'a>(
    box_solutions: &'a Vec<SortedSolution>,
) -> Vec<InvolvedSolutions<'a>> {
    let mut to_return = vec![];

    for (index, sol) in box_solutions.iter().enumerate() {
        let SortedSolution {
            location,
            solutions,
        } = sol;
        let BoxLocation {
            line,
            column,
            region,
        } = location;

        println!(
            "{} = [{}:{}]({}) -> {:?}",
            index, line, column, region, solutions
        );

        let mut involved_forward_boxes_indices = vec![];

        if index + 1 < box_solutions.len() {
            for other_box_index in (index + 1)..box_solutions.len() {
                let other_box_location = box_solutions[other_box_index].location;

                let same_line = other_box_location.line == *line;
                let same_col = other_box_location.column == *column;
                let same_region = other_box_location.region == *region;

                if same_line | same_col | same_region {
                    involved_forward_boxes_indices.push(other_box_location);
                }
            }
        }

        println!("\t{:?}", involved_forward_boxes_indices);
        let combo = InvolvedSolutions {
            location,
            solutions,
            involved_forward: involved_forward_boxes_indices,
        };

        to_return.push(combo);
    }

    to_return
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
