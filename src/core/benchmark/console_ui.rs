use super::{runner::FunctionName, time_utils::nano_to_hr};

use std::{io::stdout, time::Duration};

use ascii_table::{Align, AsciiTable};

use crossterm::{
    cursor, execute,
    style::{Print, StyledContent, Stylize},
    terminal,
};

////////////////////

pub type ColoredText = StyledContent<String>;
pub type CursorPos = (u16, u16);

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

pub fn print_table(titles: Vec<String>, data: Vec<Vec<ColoredText>>) {
    let mut ascii_table = AsciiTable::default();

    for (i, title) in titles.into_iter().enumerate() {
        ascii_table
            .column(i)
            .set_header(title)
            .set_align(Align::Center);
    }

    execute!(
        stdout(),
        Print(ascii_table.format(data)),
        Print("\n"),
        cursor::MoveToColumn(0),
    )
    .unwrap();
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

////////////////////

pub fn color_txt(param: ToColorize, color: TextColor) -> ColoredText {
    let txt = match param {
        ToColorize::Str(s) => s,
        ToColorize::Int(i) => i.to_string(),
        ToColorize::U8(u) => u.to_string(),
        ToColorize::FuncName(f) => f.to_string(),
        ToColorize::Dur(d) => nano_to_hr(d),
    };

    match color {
        TextColor::Normal => txt.reset(),
        TextColor::Green => txt.green(),
        TextColor::Yellow => txt.yellow(),
    }
}