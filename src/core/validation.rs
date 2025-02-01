use super::{
    constants::{LENGTH_DIMENSION, TO_BE_SOLVED},
    grid::{region_to_location, BoxLocation, GridValues},
};

use std::fmt;

////////////////////////////////////////

#[derive(Debug)]
pub enum ValidationErrorType {
    LINE,
    COLUMN,
    REGION,
}

#[derive(Debug)]
pub struct ValidationError {
    err_type: ValidationErrorType,
    index: usize,
    duplicated_pos: usize,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let zone = match self.err_type {
            ValidationErrorType::LINE => "Line",
            ValidationErrorType::COLUMN => "Column",
            ValidationErrorType::REGION => "Region",
        };

        write!(
            f,
            "{} index {} is not valid, duplicate value {}",
            zone, self.index, self.duplicated_pos
        )
    }
}

////////////////////////////////////////

/// Checks no duplication of values for line, columns and row
pub fn validate(values: &GridValues) -> Result<(), ValidationError> {
    for index in 0..(LENGTH_DIMENSION as usize) {
        let result = validate_new_box(
            values,
            &BoxLocation {
                line: index,
                column: index,
                region: index as u8,
            },
        );

        // Syntax magic to keep going if ok or breaking if wrong
        result?
    }

    Ok(())
}

pub fn validate_new_box(
    values: &GridValues,
    box_location: &BoxLocation,
) -> Result<(), ValidationError> {
    let BoxLocation {
        line,
        column,
        region,
    } = box_location;

    let (is_line_valid, wrong_line_value) = is_line_valid(values, line);

    if !is_line_valid {
        return Err(ValidationError {
            err_type: ValidationErrorType::LINE,
            index: *line,
            duplicated_pos: wrong_line_value.unwrap(),
        });
    }

    let (is_column_valid, wrong_column_value) = is_column_valid(values, column);

    if !is_column_valid {
        return Err(ValidationError {
            err_type: ValidationErrorType::COLUMN,
            index: *column,
            duplicated_pos: wrong_column_value.unwrap(),
        });
    }

    let (is_region_valid, wrong_region_value) = is_region_valid(values, region);

    if !is_region_valid {
        return Err(ValidationError {
            err_type: ValidationErrorType::REGION,
            index: *region as usize,
            duplicated_pos: wrong_region_value.unwrap(),
        });
    }

    Ok(())
}

fn handle_index_out_of_bound(index: &usize) {
    if *index > LENGTH_DIMENSION.into() {
        panic!("Index '{index}' out of bound");
    }
}

pub fn is_line_valid(values: &GridValues, line_index: &usize) -> (bool, Option<usize>) {
    handle_index_out_of_bound(line_index);

    let mut already_used = vec![];

    for value in values[*line_index].iter() {
        if already_used.contains(value) && *value != TO_BE_SOLVED {
            return (false, Some(*value as usize));
        } else {
            already_used.push(*value);
        }
    }

    (true, None)
}

pub fn is_column_valid(values: &GridValues, column_index: &usize) -> (bool, Option<usize>) {
    handle_index_out_of_bound(column_index);

    let mut already_used = vec![];

    for line in values.iter().take(LENGTH_DIMENSION.into()) {
        let value = line[*column_index];

        if already_used.contains(&value) && value != TO_BE_SOLVED {
            return (false, Some(value as usize));
        } else {
            already_used.push(value);
        }
    }

    (true, None)
}

pub fn is_region_valid(values: &GridValues, region_index: &u8) -> (bool, Option<usize>) {
    handle_index_out_of_bound(&(*region_index as usize));

    let (start_row, start_column) = region_to_location(region_index);
    let third_of_length = (LENGTH_DIMENSION / 3) as usize;

    let mut already_used = vec![];

    for row in values.iter().skip(start_row).take(third_of_length) {
        for value in row.iter().skip(start_column).take(third_of_length) {
            if already_used.contains(&value) && *value != TO_BE_SOLVED {
                return (false, Some(*value as usize));
            } else {
                already_used.push(value);
            }
        }
    }

    (true, None)
}
