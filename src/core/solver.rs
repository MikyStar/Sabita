use crate::core::grid::print_2d_vec;

use super::constants::{LENGTH_DIMENSION, TO_BE_SOLVED};
use super::grid::{BoxLocation, GridValues};
use super::validation::{validate, validate_new_box};

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

//////////

#[derive(Debug)]
pub struct NoSudokuSolutionFound;

impl<'a> fmt::Display for NoSudokuSolutionFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No Sudoku solution found for the provided grid")
    }
}

//////////

/// Associate a box index with it's solution index
type SolutionStore = Vec<(usize, usize)>;

////////////////////////////////////////

// TODO mais en fait, le système d'involved il sert a rien, j'econmise pas de temps à le faire puisque je
// tests dedans, faut juste iterer sur les missings avec backtracking et tester de 1 à 9

pub fn solve(
    grid_values: &GridValues,
    missing_boxes: &Vec<BoxLocation>,
) -> Result<GridValues, NoSudokuSolutionFound> {
    let mut grid_copy = grid_values.clone();

    let sols = get_solutions_complexity_sorted(grid_values, missing_boxes);
    let with_involved = get_involved_solutions(&sols);

    println!();

    let mut involved_index = 0;
    let mut store: SolutionStore = vec![];

    while involved_index < with_involved.len() {
        let box_sol = with_involved[involved_index].clone();
        let InvolvedSolutions {
            current_box:
                SortedSolution {
                    location: current_box_location,
                    solutions: current_box_solutions,
                },
            mut involved_forward,
        } = box_sol;

        let mut sol_found_index: Option<usize> = None;

        let start = match search_store(&store, involved_index) {
            Some(index) => store[index].1,
            None => 0,
        };

        println!("---------------------");
        println!("{involved_index} ==> {}", box_sol.current_box);
        // for i in involved_forward.clone() {
        //     println!("\t{i}");
        // }

        if start < current_box_solutions.len() {
            for curr_sol_index in start..current_box_solutions.len() {
                let curr_sol = &current_box_solutions[curr_sol_index];

                println!("\ttesting sol {curr_sol}");

                let affected_sol_indices = appy_sol(
                    &mut grid_copy,
                    &mut involved_forward,
                    current_box_location,
                    curr_sol,
                );

                match validate_new_box(&grid_copy, &current_box_location) {
                    Ok(_) => {
                        println!("\tgood sol");
                        sol_found_index = Some(curr_sol_index);
                        break;
                    }
                    Err(_) => {
                        if current_box_solutions.len() == 1 {
                            println!("no more sols available");
                            return Err(NoSudokuSolutionFound);
                        }

                        rollback_sol(
                            &mut grid_copy,
                            &mut involved_forward,
                            current_box_location,
                            affected_sol_indices,
                            curr_sol,
                        );
                    }
                }
            }
        } else {
            println!("skipping searching a sol");
            grid_copy[current_box_location.line as usize][current_box_location.column as usize] =
                TO_BE_SOLVED;
        }

        print_2d_vec(&grid_copy);

        match sol_found_index {
            None => {
                println!();
                println!("*** BACKWARD -> to {}", involved_index - 1);

                if involved_index == 0 {
                    println!("----------- WILL BREAK");
                    println!("{current_box_solutions:?} {store:?}");
                }

                involved_index -= 1;

                match search_store(&store, involved_index) {
                    Some(index) => {
                        let matched_val = store[index];
                        store[index].1 += 1;
                        store.retain(|&el| el.0 <= matched_val.0);
                        println!("incrementing {matched_val:?}",);
                        println!("\t{store:?}\n",);
                    }
                    None => {
                        store.push((involved_index, 1));
                        store.retain(|&el| el.0 <= involved_index);
                        println!("creating {involved_index}");
                        println!("\t{store:?}\n");
                    }
                };
            }
            Some(the_sol_index) => {
                println!();
                println!("*** FORWARD -> to {}", involved_index + 1);

                match search_store(&store, involved_index) {
                    Some(index) => {
                        let matched_val = store[index];
                        store[index].1 = the_sol_index;
                        println!("updating found index {matched_val:?}");
                        println!("\t{store:?}\n");
                    }
                    None => {
                        store.push((involved_index, the_sol_index));
                        println!("pushing ({involved_index}, {the_sol_index})");
                        println!("\t{store:?}\n");
                    }
                };

                involved_index += 1;
            }
        }
    }

    Ok(grid_copy)
}

/// Returns index in store of matching box index
fn search_store(store: &SolutionStore, box_index: usize) -> Option<usize> {
    return store.iter().position(|&r| r.0 == box_index);
}

/// Updates the grid with a value at the given position and remove that value from the involved
/// boxes possible solutions
/// Returns affected involved_boxes index
fn appy_sol(
    grid_copy: &mut GridValues,
    involved_boxes: &mut Vec<SortedSolution>,
    loc: &BoxLocation,
    value: &u8,
) -> Vec<usize> {
    grid_copy[loc.line as usize][loc.column as usize] = *value;

    // involved_boxes

    let mut affected_indices = vec![];

    for (box_index, involved) in involved_boxes.into_iter().enumerate() {
        for (sol_index, sol) in involved.solutions.clone().into_iter().enumerate() {
            if sol == *value {
                involved.solutions.remove(sol_index);
                affected_indices.push(box_index);
            }
        }
    }

    affected_indices
}

/// Updates the grid with to be solved at the given location and put back the possibility in
/// involved solutions
fn rollback_sol(
    grid_copy: &mut GridValues,
    involved_boxes: &mut Vec<SortedSolution>,
    loc: &BoxLocation,
    affected_box_solutions: Vec<usize>,
    value: &u8,
) -> () {
    grid_copy[loc.line as usize][loc.column as usize] = TO_BE_SOLVED;

    for index in affected_box_solutions {
        involved_boxes[index].solutions.push(*value);
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
