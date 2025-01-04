use super::grid::GridValues;

use std::fs::File;
use std::io::prelude::*;

////////////////////////////////////////

pub fn read(path: String) -> GridValues {
    let values: GridValues = vec![];

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{contents}");

    values
}
