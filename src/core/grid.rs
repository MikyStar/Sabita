pub const LENGTH_DIMENSION: u8 = 9;
pub const TO_BE_SOLVED: u8 = 0;

////////////////////

#[derive(Debug)]
enum BoxTypes {
    Given,
    ToBeSolved,
    Hypothesis,
}

type GridValues = Vec<Vec<u8>>;
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

                if value > &LENGTH_DIMENSION.into() {
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

        let grid = Grid { values, types };

        for index in 0..LENGTH_DIMENSION.into() {
            let (is_line_valid, wrong_line_value) = grid.is_line_valid(index);
            let (is_column_valid, wrong_column_value) = grid.is_column_valid(index);
            let (is_region_valid, wrong_region_value) = grid.is_region_valid(index);

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

        return grid;
    }

    //////////
    // Accessors

    pub fn get_values(&self) -> GridValues {
        return (*self.values).to_vec();
    }

    //////////
    // Methods

    fn is_line_valid(&self, line_index: u8) -> (bool, Option<u8>) {
        self.handle_index_out_of_bound(line_index);

        let mut already_used = vec![];

        for value in self.values[line_index as usize].iter() {
            if already_used.contains(value) && *value != TO_BE_SOLVED {
                return (false, Some(*value));
            } else {
                already_used.push(*value);
            }
        }

        return (true, None);
    }

    fn is_column_valid(&self, column_index: u8) -> (bool, Option<u8>) {
        self.handle_index_out_of_bound(column_index);

        let mut already_used = vec![];

        for index in 0..LENGTH_DIMENSION.into() {
            let value = self.values[index][column_index as usize];

            if already_used.contains(&value) && value != TO_BE_SOLVED {
                return (false, Some(value));
            } else {
                already_used.push(value);
            }
        }

        return (true, None);
    }

    fn is_region_valid(&self, region_index: u8) -> (bool, Option<u8>) {
        self.handle_index_out_of_bound(region_index);

        let (start_row, start_column) = match region_index {
            0 => (0 as u8, 0 as u8),
            1 => (0 as u8, 3 as u8),
            2 => (0 as u8, 6 as u8),
            3 => (3 as u8, 0 as u8),
            4 => (3 as u8, 3 as u8),
            5 => (3 as u8, 6 as u8),
            6 => (6 as u8, 0 as u8),
            7 => (6 as u8, 3 as u8),
            8 => (6 as u8, 6 as u8),
            _ => panic!("Region out of range"),
        };
        let third_of_length = LENGTH_DIMENSION / 3;

        let mut already_used = vec![];

        for row_index in start_row..(start_row + third_of_length) {
            for column_index in start_column..(start_column + third_of_length) {
                let value = self.values[row_index as usize][column_index as usize];

                if already_used.contains(&value) && value != TO_BE_SOLVED {
                    return (false, Some(value));
                } else {
                    already_used.push(value);
                }
            }
        }

        return (true, None);
    }

    pub fn locate_missing_box(&self) -> Vec<Vec<u8>> {
        let mut locations = vec![];

        for (row_index, row) in self.values.iter().enumerate() {
            for (column_index, value) in row.iter().enumerate() {
                if *value == 0 {
                    locations.push(vec![row_index as u8, column_index as u8]);
                }
            }
        }

        return locations;
    }

    pub fn print_grid(&self, grid: Option<GridValues>) {
        grid.unwrap_or(self.values.clone())
            .iter()
            .for_each(|line| println!("{:?}", line))
    }

    fn handle_index_out_of_bound(&self, index: u8) {
        if index > LENGTH_DIMENSION.into() {
            panic!("Index '{index}' out of bound");
        }
    }
}

////////////////////

pub fn print_grid(grid: GridValues) {
    grid.iter().for_each(|line| println!("{:?}", line))
}
