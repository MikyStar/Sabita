use super::{file::write, time_utils::nano_to_hr};

use std::{io::stdout, time::Duration};

use ascii_table::{Align, AsciiTable};
use crossterm::{
    cursor, execute,
    style::{Print, StyledContent, Stylize},
    terminal,
};
use strip_ansi_escapes::strip;

////////////////////

pub type ColoredText = StyledContent<String>;
pub type CursorPos = (u16, u16);

////////////////////

pub enum ToColorize {
    Str(String),
    Int(i32),
    U8(u8),
    Dur(Duration),
}

pub enum TextColor {
    Normal,
    Green,
    Yellow,
    Cyan,
}

////////////////////

pub fn get_cursor_position() -> CursorPos {
    cursor::position().unwrap()
}

pub fn clear_lines_from(pos: CursorPos) {
    let final_row = cursor::position().unwrap().1;
    let lines_printed = final_row.saturating_sub(pos.1);

    execute!(
        stdout(),
        cursor::MoveUp(lines_printed),
        terminal::Clear(terminal::ClearType::FromCursorDown)
    )
    .unwrap();
}

pub fn print_table(titles: Vec<String>, data: Vec<Vec<ColoredText>>, file_path: Option<String>) {
    let mut ascii_table = AsciiTable::default();

    for (i, title) in titles.into_iter().enumerate() {
        ascii_table
            .column(i)
            .set_header(title)
            .set_align(Align::Center);
    }

    let table = ascii_table.format(data);

    if let Some(ref path) = file_path {
        write(path.to_string(), vec![table.clone()]);
    }

    execute!(stdout(), Print(table), Print("\n"), cursor::MoveToColumn(0),).unwrap();
}

pub fn queue_msg(txt: String) {
    execute!(
        stdout(),
        Print(txt),
        cursor::MoveToColumn(0),
        cursor::MoveDown(1),
    )
    .unwrap();
}

pub fn color_txt(param: ToColorize, color: TextColor) -> ColoredText {
    let txt = match param {
        ToColorize::Str(s) => s,
        ToColorize::Int(i) => i.to_string(),
        ToColorize::U8(u) => u.to_string(),
        ToColorize::Dur(d) => nano_to_hr(d),
    };

    match color {
        TextColor::Normal => txt.reset(),
        TextColor::Green => txt.green(),
        TextColor::Yellow => txt.yellow(),
        TextColor::Cyan => txt.cyan(),
    }
}

pub fn get_terminal_width() -> u16 {
    terminal::size().unwrap().0
}

pub fn remove_style(text: Vec<String>) -> Vec<String> {
    text.iter()
        .map(|line| String::from_utf8(strip(line)).unwrap())
        .collect()
}
