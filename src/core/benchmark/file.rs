use super::console_ui::remove_style;

use std::{fs::OpenOptions, io::Write};

////////////////////////////////////////

pub fn write(path: String, lines: Vec<String>) {
    let content = remove_style(lines);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

    for string in content {
        writeln!(file, "{}", string).unwrap();
    }
}
