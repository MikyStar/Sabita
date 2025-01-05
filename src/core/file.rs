use super::grid::GridValues;

use std::fs;

////////////////////////////////////////

pub fn read(path: String) -> GridValues {
    let mut values: GridValues = vec![];

    let contents = fs::read_to_string(path).expect("Unable to read file '{path}'");

    let lines = contents.split("\n");

    for (line_index, line) in lines.enumerate() {
        let space_trimmed = line.replace(" ", "");
        let splitted = space_trimmed.split(",");

        let mut val_line: Vec<u8> = vec![];

        for (col_index, val) in splitted.enumerate() {
            if val.is_empty() {
                break;
            }

            match (val).parse::<u8>() {
                Ok(number) => val_line.push(number),
                Err(err) => panic!(
                    "Parsing file error, wrong value '{val}' at position [{line_index}:{col_index}]: {}",
                    err
                ),
            };
        }

        if !val_line.is_empty() {
            values.push(val_line);
        }
    }

    values
}

pub fn write(path: String, values: GridValues) {
    let mut content: String = String::new();

    for line in values {
        let val_line: String = line
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        content += &(val_line + "\n");
    }

    fs::write(path, content).expect("Unable to write into file '{path}'");
}
