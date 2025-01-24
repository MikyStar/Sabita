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
        TextColor::Cyan => txt.cyan(),
    }
}

pub fn draw_histogram(results: &BenchmarkResult) {
    let BenchmarkResult {
        fastest,
        slowest,
        average,
        times,
        ..
    } = results;

    let nb_buckets_arround = 3;

    let min_nanos = fastest.as_nanos();
    let max_nanos = slowest.as_nanos();
    let avg_nanos = average.as_nanos();

    let span_before_avg = avg_nanos - min_nanos;
    let span_after_avg = max_nanos - avg_nanos;

    let before_bucket_size = (span_before_avg / nb_buckets_arround) as usize;
    let after_bucket_size = (span_after_avg / nb_buckets_arround) as usize;

    let mut before_buckets = vec![0; before_bucket_size];
    let mut after_buckets = vec![0; after_bucket_size];

    for duration in times {
        let duration_nanos = duration.as_nanos();

        if duration_nanos <= avg_nanos {
            // let bucket_index = (duration_nanos - min_nanos) as usize / before_bucket_size;
            let bucket_index = ((duration_nanos - min_nanos) / (before_bucket_size as u128))
                .min(nb_buckets_arround as u128 - 1) as usize;
            before_buckets[bucket_index] += 1;
        } else {
            // let bucket_index = (duration_nanos - avg_nanos) as usize / after_bucket_size;
            let bucket_index = ((duration_nanos - avg_nanos) / (after_bucket_size as u128))
                .min(nb_buckets_arround as u128 - 1) as usize;

            after_buckets[bucket_index] += 1;
        }
    }

    let terminal_width = terminal::size().unwrap().0;

    let mut max_count = 0;
    for i in 0..(nb_buckets_arround as usize) {
        let before_count = before_buckets[i];
        let after_count = after_buckets[i];

        if before_count > max_count {
            max_count = before_count;
        }
        if after_count > max_count {
            max_count = after_count;
        }
    }
    let largest_count_chars = max_count.to_string().len() as u16;
    let static_delimiter = 4; // Counting spaces and |

    let mut largest_title: u16 = 0;
    let mut to_print_before: Vec<String> = vec![];
    let mut to_print_after: Vec<String> = vec![];

    for i in 0..(nb_buckets_arround as usize) {
        // TODO refactor duplication between buckets
        let range_before_start = min_nanos + (before_bucket_size as u128) * (i as u128);
        let range_before_end = range_before_start + (before_bucket_size as u128);
        let range_after_start = avg_nanos + (after_bucket_size as u128) * (i as u128);
        let range_after_end = range_after_start + (after_bucket_size as u128);

        let start_before = nano_to_hr(Duration::from_nanos(range_before_start.try_into().unwrap()));
        let end_before = nano_to_hr(Duration::from_nanos(range_before_end.try_into().unwrap()));
        let start_after = nano_to_hr(Duration::from_nanos(range_after_start.try_into().unwrap()));
        let end_after = nano_to_hr(Duration::from_nanos(range_after_end.try_into().unwrap()));

        let line_before = format!("{start_before} - {end_before}");
        let line_after = format!("{start_after} - {end_after}");

        if line_before.len() > (largest_title as usize) {
            largest_title = line_before.len() as u16;
        }
        if line_after.len() > (largest_title as usize) {
            largest_title = line_after.len() as u16;
        }

        to_print_before.push(line_before);
        to_print_after.push(line_after);
    }

    // TODO refactor
    for (i, time_txt) in to_print_before.into_iter().enumerate() {
        let count = before_buckets[i];
        let pct = (count * 100) as f64 / times.len() as f64;

        let nb_spaces_needed = largest_title - (time_txt.len() as u16);
        let spaces = " ".repeat(nb_spaces_needed as usize);

        let available_space_for_bar =
            terminal_width - (largest_title + largest_count_chars + static_delimiter);

        let nb_bars = ((count as u16) * (available_space_for_bar) / (max_count as u16)) as usize;
        let bar = "█".repeat(nb_bars);

        let space_before = match nb_bars > 0 {
            true => " ",
            false => "",
        };

        let line = format!("{time_txt}{spaces} │ {bar}{space_before}{pct}%");
        println!("{line}");
    }

    let avg_txt = format!(
        "  Average {}",
        nano_to_hr(Duration::from_nanos(avg_nanos.try_into().unwrap()))
    );
    let nb_spaces_needed = largest_title - (avg_txt.len() as u16);
    let spaces = " ".repeat(nb_spaces_needed as usize);

    let available_space_for_bar =
        (terminal_width - (largest_title + largest_count_chars) + 1) as usize;

    let bar = "─".repeat(available_space_for_bar / 2);
    let line = color_txt(
        ToColorize::Str(format!("{avg_txt}{spaces}├{bar}")),
        TextColor::Yellow,
    );
    println!("{line}");

    for (i, time_txt) in to_print_after.into_iter().enumerate() {
        let count = after_buckets[i];
        let pct = (count * 100) as f64 / times.len() as f64;

        let nb_spaces_needed = largest_title - (time_txt.len() as u16);
        let spaces = " ".repeat(nb_spaces_needed as usize);

        let available_space_for_bar =
            terminal_width - (largest_title + largest_count_chars + static_delimiter);

        let nb_bars = ((count as u16) * (available_space_for_bar) / (max_count as u16)) as usize;
        let bar = "█".repeat(nb_bars);

        let space_before = match nb_bars > 0 {
            true => " ",
            false => "",
        };

        let line = format!("{time_txt}{spaces} │ {bar}{space_before}{pct}%");
        println!("{line}");
    }
}
