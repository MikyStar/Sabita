use super::runner::FunctionName;

use std::{io::stdout, time::Duration};

use humanize_duration::{prelude::DurationExt, Truncate};

use ascii_table::{Align, AsciiTable};

use crossterm::{
    cursor, execute,
    style::{StyledContent, Stylize},
    terminal,
};

////////////////////

pub type ColoredText = StyledContent<String>;

////////////////////

pub enum ToColorize {
    Str(String),
    Int(i32),
    U8(u8),
    FuncName(FunctionName),
    Dur(Duration),
}

pub enum TextColor {
    Normal,
    Green,
    Yellow,
}

////////////////////

pub fn clean_last_rows(count: u16) {
    execute!(
        stdout(),
        cursor::MoveToColumn(0),
        cursor::MoveUp(count),
        terminal::Clear(terminal::ClearType::FromCursorDown),
    )
    .unwrap();
}

pub fn print_table(titles: Vec<String>, data: Vec<Vec<ColoredText>>) {
    let mut ascii_table = AsciiTable::default();

    for (i, title) in titles.into_iter().enumerate() {
        ascii_table
            .column(i)
            .set_header(title)
            .set_align(Align::Center);
    }

    ascii_table.print(data);
}

////////////////////

pub fn color_txt(param: ToColorize, color: TextColor) -> ColoredText {
    let txt = match param {
        ToColorize::Str(s) => s,
        ToColorize::Int(i) => i.to_string(),
        ToColorize::U8(u) => u.to_string(),
        ToColorize::FuncName(f) => f.to_string(),
        ToColorize::Dur(d) => d.human(Truncate::Nano).to_string(),
    };

    match color {
        TextColor::Normal => txt.reset(),
        TextColor::Green => txt.green(),
        TextColor::Yellow => txt.yellow(),
    }
}
