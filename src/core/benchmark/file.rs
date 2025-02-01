use super::console_ui::{
    clear_lines_from, color_txt, get_cursor_position, remove_style, TextColor, ToColorize,
};

use std::{
    fs::{remove_file, OpenOptions},
    io::{stdin, Write},
    path::Path,
    process::exit,
};

////////////////////////////////////////

pub enum FilePolicy {
    Append,
    Rewrite,
    Cancel,
}

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

pub fn handle_file(path: String, policy: Option<FilePolicy>) {
    if does_file_exists(path.clone()) {
        match policy {
            Some(pol) => match pol {
                FilePolicy::Append => {
                    write(path, vec!["".to_string(), "-".repeat(20), "".to_string()])
                }
                FilePolicy::Rewrite => remove_file(path).unwrap(),
                FilePolicy::Cancel => exit(0),
            },
            None => {
                let cursor_pos = get_cursor_position();

                println!("File '{path}' already exists");
                println!(
                    "({})ppend to it, ({})ewrite it, ({})ancel ?",
                    color_txt(ToColorize::Str("a".to_string()), TextColor::Green),
                    color_txt(ToColorize::Str("r".to_string()), TextColor::Green),
                    color_txt(ToColorize::Str("c".to_string()), TextColor::Green),
                );

                let mut prompt = String::new();
                stdin().read_line(&mut prompt).expect("Failed to read line");

                match prompt.as_str().trim() {
                    "a" => write(path, vec!["".to_string(), "-".repeat(20), "".to_string()]),
                    "r" => remove_file(path).unwrap(),
                    "c" => exit(0),
                    _ => panic!("Invalid prompt '{prompt}', should be either a, r or c"),
                }

                clear_lines_from(cursor_pos);
            }
        }
    }
}
