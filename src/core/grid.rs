use crate::{assets::full_grid::ConstGridValues, utils::grid_utils::grid_values_array_to_vec};

use super::{
    constants::{LENGTH_DIMENSION, MAX_NB_VALUES, MINIMUM_PROVIDED},
    file::{read, write},
    generator::{generate, remove_random_values},
    solver::{locate_missing_box, solve},
    validation::validate,
};

use std::{error::Error, fmt};

////////////////////////////////////////

pub type GridValues = Vec<Vec<u8>>;

#[derive(Debug, Clone)]
pub struct BoxLocation {
    pub line: usize,
    pub column: usize,
    pub region: u8,
}

/// Format how printing with only {} does
/// Otherwise you'll need to use {:?} to print the structure in a single lie
/// Or {:#?} to 'pretty' print each attribute on it's own line
impl fmt::Display for BoxLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}:{}]({})", self.line, self.column, self.region)
    }
}

impl PartialEq for BoxLocation {
    fn eq(&self, other: &Self) -> bool {
        let same_line = self.line == other.line;
        let same_column = self.column == other.column;
        let same_region = self.region == other.region;

        same_line & same_column & same_region
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub values: GridValues,
}

////////////////////

impl Grid {
    //////////
    // Constructor

    pub fn new(values: GridValues) -> Self {
        if values.len() != LENGTH_DIMENSION.into() {
            panic!("Wrong number of lines: {}", values.len());
        }

        for (row_index, row) in values.iter().enumerate() {
            if row.len() != LENGTH_DIMENSION.into() {
                panic!(
                    "Line index {} has a different number of columns than {}",
                    row_index, LENGTH_DIMENSION
                )
            }

            for (column_index, value) in row.iter().enumerate() {
                if row.len() > LENGTH_DIMENSION.into() {
                    panic!(
                        "Column index {} larger than {}",
                        column_index, LENGTH_DIMENSION
                    )
                }

                if value > &LENGTH_DIMENSION {
                    panic!(
                        "Value '{}' out of bound at position {};{}",
                        value, row_index, column_index
                    );
                }
            }
        }

        match validate(&values) {
            Ok(_) => {}
            Err(err) => panic!("{err}"),
        }

        Grid { values }
    }

    pub fn generate(nb_to_remove: Option<u8>) -> Self {
        let mut values = generate().unwrap();

        if let Some(to_remove) = nb_to_remove {
            let remaining_cells = MAX_NB_VALUES - to_remove;

            if remaining_cells < MINIMUM_PROVIDED {
                println!("Carefull, {MINIMUM_PROVIDED} is considered the minimum number of values to provide to solve a sudoku");
            }

            values = remove_random_values(&values, to_remove).0;
        }

        Grid::new(values)
    }

    pub fn from_array(array: ConstGridValues) -> Self {
        let values = grid_values_array_to_vec(array);

        Grid::new(values)
    }

    pub fn from_file(path: String) -> Self {
        let values = read(path);

        Grid::new(values)
    }

    //////////
    // Accessors

    pub fn get_values(&self) -> GridValues {
        (*self.values).to_vec()
    }

    //////////
    // Methods

    pub fn locate_missing_box(&self) -> Vec<BoxLocation> {
        locate_missing_box(&self.get_values())
    }

    pub fn remove_random_values(&mut self, nb_to_remove: u8) -> Vec<BoxLocation> {
        let (values, locations) = remove_random_values(&self.values, nb_to_remove);

        self.values = values;

        locations
    }

    pub fn print(&self) {
        print_2d_vec(&self.get_values());
    }

    pub fn solve(&mut self) {
        let missing_boxes = self.locate_missing_box();

        let values = solve(&self.get_values(), &missing_boxes).unwrap();

        self.values = values;
    }

    pub fn dump_file(&self, path: String) {
        write(path, self.get_values());
    }
}

////////////////////

/// Prints a two dimensions vector to stdout
pub fn print_2d_vec(grid: &GridValues) {
    grid.iter().for_each(|line| println!("{:?}", line))
}

/// Parse coordinates (line, column) into a region index
pub fn location_to_region(line: &usize, col: &usize) -> Result<u8, Box<dyn Error>> {
    let third_of_length = (LENGTH_DIMENSION / 3) as usize;

    for index in 0..LENGTH_DIMENSION {
        let (start_row, start_column) = region_to_location(&index);

        let end_row = start_row + third_of_length - 1;
        let end_column = start_column + third_of_length - 1;

        let is_in_region_row = line >= &start_row && line <= &end_row;
        let is_in_region_column = col >= &start_column && col <= &end_column;

        if is_in_region_row & is_in_region_column {
            return Ok(index);
        }
    }

    Err(format!("No region found for [{},{}]", line, col).into())
}

/// Parse a region index into coordinates (line, column)
pub fn region_to_location(region_index: &u8) -> (usize, usize) {
    match region_index {
        0 => (0_usize, 0_usize),
        1 => (0_usize, 3_usize),
        2 => (0_usize, 6_usize),
        3 => (3_usize, 0_usize),
        4 => (3_usize, 3_usize),
        5 => (3_usize, 6_usize),
        6 => (6_usize, 0_usize),
        7 => (6_usize, 3_usize),
        8 => (6_usize, 6_usize),
        _ => panic!("Region out of range"),
    }
}
