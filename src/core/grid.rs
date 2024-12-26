use super::constants::LENGTH_DIMENSION;
use super::validation::{is_column_valid, is_line_valid, is_region_valid};

use std::error::Error;
use std::fmt;

////////////////////////////////////////

#[derive(Debug)]
enum BoxTypes {
    Given,
    ToBeSolved,
    Hypothesis,
}

pub type GridValues = Vec<Vec<u8>>;
type GridTypes = Vec<Vec<BoxTypes>>;

#[derive(Debug, Clone)]
pub struct BoxLocation {
    pub line: u8,
    pub column: u8,
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

#[derive(Debug)]
pub struct Grid {
    values: GridValues,
    types: GridTypes,
}

////////////////////

impl Grid {
    //////////
    // Constructor

    pub fn new(values: GridValues) -> Self {
        let mut types: GridTypes = vec![];

        if values.len() != LENGTH_DIMENSION.into() {
            panic!("Wrong number of lines: {}", values.len());
        }

        for (row_index, row) in values.iter().enumerate() {
            if row.len() != LENGTH_DIMENSION.into() {
                panic!(
                    "Row index {} has a different number of columns than {}",
                    row_index, LENGTH_DIMENSION
                )
            }

            let mut line = vec![];

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

                line.push(if *value > 0 {
                    BoxTypes::Given
                } else {
                    BoxTypes::ToBeSolved
                });
            }

            types.push(line);
        }

        for index in 0..LENGTH_DIMENSION {
            let (is_line_valid, wrong_line_value) = is_line_valid(&values, &index);
            let (is_column_valid, wrong_column_value) = is_column_valid(&values, &index);
            let (is_region_valid, wrong_region_value) = is_region_valid(&values, &index);

            if !is_line_valid {
                panic!(
                    "Row index {} is not valid, duplicate value {}",
                    index,
                    wrong_line_value.unwrap()
                );
            }

            if !is_column_valid {
                panic!(
                    "Column index {} is not valid, duplicate value {}",
                    index,
                    wrong_column_value.unwrap()
                );
            }

            if !is_region_valid {
                panic!(
                    "Region index {} is not valid, duplicate value {}",
                    index,
                    wrong_region_value.unwrap()
                );
            }
        }

        Grid { values, types }
    }

    //////////
    // Accessors

    pub fn get_values(&self) -> GridValues {
        (*self.values).to_vec()
    }

    //////////
    // Methods

    pub fn locate_missing_box(&self) -> Vec<BoxLocation> {
        let mut locations = vec![];

        for (row_index, row) in self.values.iter().enumerate() {
            for (column_index, value) in row.iter().enumerate() {
                if *value == 0 {
                    let line = row_index as u8;
                    let column = column_index as u8;
                    let region = location_to_region(&line, &column).unwrap();

                    let loc = BoxLocation {
                        line,
                        column,
                        region,
                    };

                    locations.push(loc);
                }
            }
        }

        locations
    }

    pub fn print(&self) {
        print_2d_vec(&self.get_values());
    }
}

////////////////////

/// Prints a two dimensions vector to stdout
pub fn print_2d_vec(grid: &GridValues) {
    grid.iter().for_each(|line| println!("{:?}", line))
}

/// Parse coordinates (line, column) into a region index
pub fn location_to_region(line: &u8, col: &u8) -> Result<u8, Box<dyn Error>> {
    let third_of_length = LENGTH_DIMENSION / 3;

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
pub fn region_to_location(region_index: &u8) -> (u8, u8) {
    let location = match region_index {
        0 => (0_u8, 0_u8),
        1 => (0_u8, 3_u8),
        2 => (0_u8, 6_u8),
        3 => (3_u8, 0_u8),
        4 => (3_u8, 3_u8),
        5 => (3_u8, 6_u8),
        6 => (6_u8, 0_u8),
        7 => (6_u8, 3_u8),
        8 => (6_u8, 6_u8),
        _ => panic!("Region out of range"),
    };

    location
}
