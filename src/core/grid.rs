use super::constants::LENGTH_DIMENSION;
use super::validation::{is_column_valid, is_line_valid, is_region_valid};

////////////////////////////////////////

#[derive(Debug)]
enum BoxTypes {
    Given,
    ToBeSolved,
    Hypothesis,
}

pub type GridValues = Vec<Vec<u8>>;
type GridTypes = Vec<Vec<BoxTypes>>;

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

    pub fn locate_missing_box(&self) -> Vec<Vec<u8>> {
        let mut locations = vec![];

        for (row_index, row) in self.values.iter().enumerate() {
            for (column_index, value) in row.iter().enumerate() {
                if *value == 0 {
                    locations.push(vec![row_index as u8, column_index as u8]);
                }
            }
        }

        locations
    }

    pub fn print_grid(&self, grid: Option<GridValues>) {
        grid.unwrap_or(self.values.clone())
            .iter()
            .for_each(|line| println!("{:?}", line))
    }
}

////////////////////

pub fn print_grid(grid: &GridValues) {
    grid.iter().for_each(|line| println!("{:?}", line))
}
