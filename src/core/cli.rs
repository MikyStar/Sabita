use std::{env, fmt, path::Path};

////////////////////////////////////////

#[derive(Debug)]
pub enum ACTION {
    Generate,
    Solve,

    HelpGenerate,
    HelpSolve,
    HelpFull,

    Version,
}

#[derive(Debug)]
pub struct ArgParsed {
    pub action: ACTION,
    pub path: Option<String>,
    pub nb_missing: Option<u8>,
}

impl fmt::Display for ArgParsed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let action = match self.action {
            ACTION::Generate => "generate",
            ACTION::Solve => "solve",

            ACTION::HelpGenerate => "help generate",
            ACTION::HelpSolve => "help solve",
            ACTION::HelpFull => "help full",

            ACTION::Version => "version",
        };

        let path = match &self.path {
            Some(val) => val.clone(),
            None => "[none]".to_string(),
        };

        let nb_missing = match &self.nb_missing {
            Some(val) => val.to_string(),
            None => "[none]".to_string(),
        };

        write!(f, "action: {action}; path: {path}; nb_missing {nb_missing}")
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
            nb_missing: None,
        };
    }

    match args[1].as_str() {
        "g" => {
            if args.len() < 3 {
                return ArgParsed {
                    action: ACTION::HelpGenerate,
                    path: None,
                    nb_missing: None,
                };
            }
            let file_path = args[2].clone();

            if Path::new(&file_path).exists() {
                panic!("Path '{file_path}' already exists");
            }

            let mut nb_missing = None;

            if args.len() == 4 {
                match (args[3]).parse::<u8>() {
                    Ok(number) => nb_missing = Some(number),
                    Err(err) => panic!("Wrong number of box to remove: {}", err),
                };
            }

            ArgParsed {
                action: ACTION::Generate,
                path: Some(file_path),
                nb_missing,
            }
        }
        "s" => {
            if args.len() != 3 {
                return ArgParsed {
                    action: ACTION::HelpSolve,
                    path: None,
                    nb_missing: None,
                };
            }

            let file_path = args[2].clone();

            if !Path::new(&file_path).exists() {
                panic!("Path '{file_path}' doesn't exists");
            }

            ArgParsed {
                action: ACTION::Solve,
                path: Some(file_path),
                nb_missing: None,
            }
        }
        "-v" | "--version" => ArgParsed {
            action: ACTION::Version,
            path: None,
            nb_missing: None,
        },
        _ => ArgParsed {
            action: ACTION::HelpFull,
            path: None,
            nb_missing: None,
        },
    }
}
