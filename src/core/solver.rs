use crate::core::grid::print_2d_vec;

use super::constants::{LENGTH_DIMENSION, TO_BE_SOLVED};
use super::grid::{BoxLocation, GridValues};
use super::validation::validate;

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
    involved_forward: Vec<SortedSolution<'a>>,
}

////////////////////////////////////////

pub fn solve(grid_values: &GridValues, missing_boxes: &Vec<BoxLocation>) {
    let mut grid_copy = grid_values.clone();

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
        let mut involved_forward_copy = involved_forward;

        for curr_sol in current_box_solutions {
            println!("before");
            print_2d_vec(&grid_copy);
            println!("{current_box_location} {curr_sol}");
            for i in involved_forward_copy.clone() {
                println!("\t{i}");
            }

            appy_sol(
                &mut grid_copy,
                &mut involved_forward_copy,
                current_box_location,
                curr_sol,
            );

            println!("after");
            print_2d_vec(&grid_copy);
            println!("{current_box_location} {curr_sol}");
            for i in involved_forward_copy.clone() {
                println!("\t{i}");
            }
        }
    }
}

/// Updates the grid with a value at the given position and remove that value from the involved
/// boxes possible solutions
fn appy_sol(
    grid_copy: &mut GridValues,
    involved_boxes: &mut Vec<SortedSolution>,
    loc: &BoxLocation,
    value: &u8,
) -> () {
    grid_copy[loc.line as usize][loc.column as usize] = *value;

    for involved in involved_boxes {
        involved.solutions.retain(|x| *x != *value);
    }
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
                    involved_forward.push(other_box.clone());
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

        match validate(&grid_to_test) {
            Ok(_) => {
                answers.push(possibility);
            }
            Err(_) => {}
        }
    }

    if answers.is_empty() {
        Err(BoxSolutionNotFound)
    } else {
        Ok(answers)
    }
}
