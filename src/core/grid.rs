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
    pub fn new(values: GridValues) -> Self {
        let mut types: GridTypes = vec![];

        for (row_index, row) in values.iter().enumerate() {
            if row.len() > LENGTH_DIMENSION.into() {
                panic!("Row index {} larger than {}", row_index, LENGTH_DIMENSION)
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
            2 => (0 as u8, 5 as u8),
            3 => (3 as u8, 0 as u8),
            4 => (3 as u8, 3 as u8),
            5 => (3 as u8, 5 as u8),
            6 => (5 as u8, 0 as u8),
            7 => (5 as u8, 3 as u8),
            8 => (5 as u8, 5 as u8),
            _ => panic!("Region out of range"),
        };

        let mut already_used = vec![];

        for row_index in start_row..(LENGTH_DIMENSION / 3) {
            for column_index in start_column..(LENGTH_DIMENSION / 3) {
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
