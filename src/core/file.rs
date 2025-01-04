use super::grid::GridValues;

use std::fs::File;
use std::io::prelude::*;

////////////////////////////////////////

pub fn read(path: String) -> GridValues {
    let mut values: GridValues = vec![];

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines = contents.split("\n");

    for (line_index, line) in lines.enumerate() {
        let space_trimmed = line.replace(" ", "");
        let splitted = space_trimmed.split(",");

        let mut val_line: Vec<u8> = vec![];

        for (col_index, val) in splitted.enumerate() {
            if val == "" {
                break;
            }

            match (&val).parse::<u8>() {
                Ok(number) => val_line.push(number),
                Err(err) => panic!(
                    "Parsing file error, wrong value '{val}' at position [{line_index}:{col_index}]: {}",
                    err
                ),
            };
        }

        if val_line.len() > 0 {
            values.push(val_line);
        }
    }

    values
}
