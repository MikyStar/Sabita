use crate::core::benchmark::runner::BenchmarkResult;

use super::{runner::FunctionName, time_utils::nano_to_hr};

use std::{io::stdout, time::Duration, usize};

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

pub fn draw_histogram(results: &BenchmarkResult, num_bins: usize) {
    let BenchmarkResult {
        fastest,
        slowest,
        average,
        std_dev,
        times,
    } = results;

    println!("{times:?}\n");

    // TODO centered value should be mean and each bucket should be based on the std-dev

    let min_nanos = fastest.as_nanos();
    let max_nanos = slowest.as_nanos();
    let avg_nanos = average.as_nanos();

    let range = max_nanos - min_nanos;
    let bucket_size = range / (num_bins as u128);

    let mut buckets = vec![0; num_bins];

    for duration in times {
        let duration_nanos = duration.as_nanos();
        let bin_index =
            ((duration_nanos - min_nanos) / bucket_size).min((num_bins as u128) - 1) as usize;

        buckets[bin_index] += 1;
    }

    let terminal_width = terminal::size().unwrap().0;

    let max_count = *buckets.iter().max().unwrap_or(&1);

    let mut to_print: Vec<String> = vec![];
    let mut largest_title: u16 = 0;

    for i in 0..buckets.len() {
        let range_start = min_nanos + bucket_size * (i as u128);
        let range_end = range_start + bucket_size;

        let start = nano_to_hr(Duration::from_nanos(range_start.try_into().unwrap()));
        let end = nano_to_hr(Duration::from_nanos(range_end.try_into().unwrap()));

        let is_average_within = {
            let is_after_start = avg_nanos >= range_start;
            let is_before_end = avg_nanos <= range_end;

            is_after_start & is_before_end
        };

        let line = format!("{start} - {end}");

        if line.len() > (largest_title as usize) {
            largest_title = line.len() as u16;
        }

        to_print.push(line);
    }

    for (i, time_range) in to_print.into_iter().enumerate() {
        let count = buckets[i];

        let nb_spaces_needed = largest_title - (time_range.len() as u16);
        let spaces = " ".repeat(nb_spaces_needed as usize);

        let largest_count_chars = max_count.to_string().len();
        let static_delimiter = 4; // Counting spaces and |
        let available_space_for_bar =
            terminal_width - (largest_title + (largest_count_chars as u16) + static_delimiter);

        let bar =
            "█".repeat(((count as u16) * (available_space_for_bar) / (max_count as u16)) as usize);

        let line = format!("{time_range}{spaces} | {bar} {count}");
        println!("{line}");
    }
}
