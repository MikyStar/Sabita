use core::panic;
use std::env;
use std::fmt;
use std::path;
use std::path::Path;

////////////////////////////////////////

#[derive(Debug)]
pub enum ACTION {
    Generate,
    Solve,

    HelpGenerate,
    HelpSolve,
    HelpFull,
}

#[derive(Debug)]
pub struct ArgParsed {
    action: ACTION,
    path: Option<String>,
}

impl fmt::Display for ArgParsed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let action = match self.action {
            ACTION::Generate => "generate",
            ACTION::Solve => "solve",

            ACTION::HelpGenerate => "help generate",
            ACTION::HelpSolve => "help solve",
            ACTION::HelpFull => "help full",
        };

        let path = match &self.path {
            Some(val) => val.clone(),
            None => "[none]".to_string(),
        };

        write!(f, "action: {action}: path: {path}")
    }
}

////////////////////////////////////////

/// Generates a sudoku
pub fn parse_args() -> ArgParsed {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return ArgParsed {
            action: ACTION::HelpFull,
            path: None,
        };
    }

    match args[1].as_str() {
        "g" => {
            if args.len() != 3 {
                return ArgParsed {
                    action: ACTION::HelpGenerate,
                    path: None,
                };
            }
            let file_path = args[2].clone();

            panic_bad_path(&file_path);

            ArgParsed {
                action: ACTION::Generate,
                path: Some(file_path),
            }
        }
        "s" => {
            if args.len() != 3 {
                return ArgParsed {
                    action: ACTION::HelpSolve,
                    path: None,
                };
            }

            let file_path = args[2].clone();

            panic_bad_path(&file_path);

            ArgParsed {
                action: ACTION::Solve,
                path: Some(file_path),
            }
        }
        _ => ArgParsed {
            action: ACTION::HelpFull,
            path: None,
        },
    }
}

////////////////////

fn panic_bad_path(path: &String) {
    if !Path::new(&path).exists() {
        panic!("Path '{path}' doesn't exists");
    }
}
