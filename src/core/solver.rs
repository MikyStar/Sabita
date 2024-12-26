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

//////////

#[derive(Debug, Clone)]
pub struct SortedSolution<'a> {
    location: &'a BoxLocation,
    solutions: Vec<u8>,
}

impl<'a> fmt::Display for SortedSolution<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {:?}", self.location, self.solutions)
    }
}

//////////

#[derive(Debug, Clone)]
pub struct InvolvedSolutions<'a> {
    current_box: &'a SortedSolution<'a>,
    involved_forward: Vec<&'a SortedSolution<'a>>,
}

////////////////////////////////////////

pub fn reduce_solutions(grid_values: &GridValues, missing_boxes: &Vec<BoxLocation>) {
    let mut grid_copy = grid_values.clone();
    let mut missing_boxes_copy = missing_boxes.clone();

    let sols = get_solutions_complexity_sorted(grid_values, missing_boxes);
    let with_involved = get_involved_solutions(&sols);

    println!();

    for box_sol in with_involved {
        let InvolvedSolutions {
            current_box:
                SortedSolution {
                    location: current_box_location,
                    solutions: current_box_solutions,
                },
            involved_forward,
        } = box_sol;

        let mut not_intersecting_solutions = vec![];

        // Skip unnecessary involvement checks
        if current_box_solutions.len() <= 1 {
            continue;
        }

        for current_box_solution in current_box_solutions {
            for forward_box in involved_forward.clone() {
                let SortedSolution {
                    solutions: forward_box_solutions,
                    ..
                } = forward_box;

                let is_solution_also_in_forward =
                    forward_box_solutions.contains(&current_box_solution);

                let already_noted = not_intersecting_solutions.contains(&current_box_solution);

                if !is_solution_also_in_forward & !already_noted {
                    println!(
                        "box {}, sol {}, notIn {:?}",
                        current_box_location, current_box_solution, forward_box_solutions
                    );
                    not_intersecting_solutions.push(current_box_solution);
                }
            }
        }
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

pub fn sort_involved_solutions<'a>(
    box_with_involved: Vec<InvolvedSolutions<'a>>,
) -> Vec<InvolvedSolutions<'a>> {
    // TODO after solutions reduced
    // TODO group involved by number of solutions (asc) and by number of involved boxes

    // TODO maybe this function makes the first sorting (get_solutions_complexity_sorted) useless and
    // maybe only this one is needed

    unimplemented!()
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
            location:
                BoxLocation {
                    line,
                    column,
                    region,
                },
            ..
        } = sol;

        println!("{index}: {sol}");

        let mut involved_forward = vec![];

        if index + 1 < box_solutions.len() {
            for other_box_index in (index + 1)..box_solutions.len() {
                let other_box = &box_solutions[other_box_index];
                let other_box_location = other_box.location;

                let same_line = other_box_location.line == *line;
                let same_col = other_box_location.column == *column;
                let same_region = other_box_location.region == *region;

                if same_line | same_col | same_region {
                    involved_forward.push(other_box);
                }
            }
        }

        for i in involved_forward.clone() {
            println!("\t{i}");
        }

        let combo = InvolvedSolutions {
            current_box: sol,
            involved_forward,
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
