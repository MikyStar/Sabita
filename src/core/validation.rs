use super::constants::{LENGTH_DIMENSION, TO_BE_SOLVED};
use super::grid::GridValues;

////////////////////////////////////////

fn handle_index_out_of_bound(index: &u8) {
    if *index > LENGTH_DIMENSION {
        panic!("Index '{index}' out of bound");
    }
}

pub fn is_line_valid(values: &GridValues, line_index: &u8) -> (bool, Option<u8>) {
    handle_index_out_of_bound(line_index);

    let mut already_used = vec![];

    for value in values[*line_index as usize].iter() {
        if already_used.contains(value) && *value != TO_BE_SOLVED {
            return (false, Some(*value));
        } else {
            already_used.push(*value);
        }
    }

    return (true, None);
}

pub fn is_column_valid(values: &GridValues, column_index: &u8) -> (bool, Option<u8>) {
    handle_index_out_of_bound(column_index);

    let mut already_used = vec![];

    for index in 0..LENGTH_DIMENSION.into() {
        let value = values[index][*column_index as usize];

        if already_used.contains(&value) && value != TO_BE_SOLVED {
            return (false, Some(value));
        } else {
            already_used.push(value);
        }
    }

    return (true, None);
}

pub fn is_region_valid(values: &GridValues, region_index: &u8) -> (bool, Option<u8>) {
    handle_index_out_of_bound(region_index);

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
            let value = values[row_index as usize][column_index as usize];

            if already_used.contains(&value) && value != TO_BE_SOLVED {
                return (false, Some(value));
            } else {
                already_used.push(value);
            }
        }
    }

    return (true, None);
}
