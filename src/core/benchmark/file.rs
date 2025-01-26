use super::{
    config::BENCH_FILE,
    console_ui::{
        clear_lines_from, color_txt, get_cursor_position, remove_style, TextColor, ToColorize,
    },
};

use std::{
    fs::{remove_file, OpenOptions},
    io::{stdin, Write},
    path::Path,
    process::exit,
};

////////////////////////////////////////

fn does_file_exists(path: String) -> bool {
    Path::new(&path).exists()
}

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

pub fn handle_file() {
    if does_file_exists(BENCH_FILE.to_string()) {
        let cursor_pos = get_cursor_position();

        println!("File '{BENCH_FILE}' already exists");
        println!(
            "({})ppend to it, ({})ewrite it, ({})ancel ?",
            color_txt(ToColorize::Str("a".to_string()), TextColor::Green),
            color_txt(ToColorize::Str("r".to_string()), TextColor::Green),
            color_txt(ToColorize::Str("c".to_string()), TextColor::Green),
        );

        let mut prompt = String::new();

        stdin().read_line(&mut prompt).expect("Failed to read line");

        match prompt.as_str().trim() {
            "a" => {
                write(
                    BENCH_FILE.to_string(),
                    vec!["".to_string(), "-".repeat(20), "".to_string()],
                );
            }
            "r" => remove_file(BENCH_FILE).unwrap(),
            "c" => exit(0),
            _ => panic!("Invalid prompt '{prompt}', should be either a, r or c"),
        }

        clear_lines_from(cursor_pos);
    }
}
