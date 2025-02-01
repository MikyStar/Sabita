use std::{cmp::max, time::Duration};

use super::{
    console_ui::{color_txt, get_terminal_width, TextColor, ToColorize},
    file::write,
    time_utils::nano_to_hr,
};
use crate::core::benchmark::runner::BenchmarkResult;

////////////////////////////////////////

const LINE_DELIMITER_COUNT: u16 = 4; // Counting spaces and |

////////////////////////////////////////

pub fn draw_histogram(
    results: &BenchmarkResult,
    nb_buckets_arround: u128,
    file_path: Option<String>,
) {
    let BenchmarkResult {
        fastest,
        slowest,
        average,
        times,
        ..
    } = results;

    let min_nanos = fastest.as_nanos();
    let max_nanos = slowest.as_nanos();
    let avg_nanos = average.as_nanos();

    // Bucket computation

    let span_before_avg = avg_nanos - min_nanos;
    let span_after_avg = max_nanos - avg_nanos;

    let before_bucket_size = span_before_avg.checked_div(nb_buckets_arround).unwrap();
    let after_bucket_size = span_after_avg.checked_div(nb_buckets_arround).unwrap();

    let mut before_buckets = vec![0; nb_buckets_arround as usize];
    let mut after_buckets = vec![0; nb_buckets_arround as usize];

    for duration in times {
        let duration_nanos = duration.as_nanos();

        if duration_nanos <= avg_nanos {
            let index = find_bucket_index(
                duration_nanos,
                min_nanos,
                before_bucket_size,
                nb_buckets_arround,
            );

            before_buckets[index] += 1;
        } else {
            let index = find_bucket_index(
                duration_nanos,
                avg_nanos,
                after_bucket_size,
                nb_buckets_arround,
            );

            after_buckets[index] += 1;
        }
    }

    let terminal_width = get_terminal_width();

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

    // Print lines computation

    let largest_count_chars = max_count.to_string().len() as u16;

    let (time_txt_before, largest_title_before) =
        compute_time_range(nb_buckets_arround, before_bucket_size, min_nanos);
    let (time_text_after, largest_title_after) =
        compute_time_range(nb_buckets_arround, after_bucket_size, avg_nanos);

    let largest_title: u16 = max(largest_title_before, largest_title_after);

    let full_line_before = compute_rest_of_line(
        time_txt_before,
        before_buckets,
        largest_title,
        times.len() as f64,
        terminal_width,
        largest_count_chars,
        max_count as u16,
    );
    let full_line_after = compute_rest_of_line(
        time_text_after,
        after_buckets,
        largest_title,
        times.len() as f64,
        terminal_width,
        largest_count_chars,
        max_count as u16,
    );

    let avg_line = compute_avg_line(
        avg_nanos,
        largest_title,
        largest_count_chars,
        terminal_width,
    );

    // Printing

    if let Some(ref path) = file_path {
        write(path.to_string(), full_line_before.clone());
    }
    for line in full_line_before {
        println!("{line}");
    }

    if let Some(ref path) = file_path {
        write(path.to_string(), vec![avg_line.clone()]);
    }
    println!("{avg_line}");

    if let Some(ref path) = file_path {
        write(path.to_string(), full_line_after.clone());
    }
    for line in full_line_after {
        println!("{line}");
    }
}

////////////////////

fn find_bucket_index(
    duration_ns: u128,
    lowest_ns: u128,
    bucket_size: u128,
    nb_buckets_arround: u128,
) -> usize {
    let bucket_index = ((duration_ns - lowest_ns) / bucket_size).min(nb_buckets_arround - 1);

    usize::try_from(bucket_index).expect("Overflow: value is too large for usize")
}

fn compute_time_range(nb_buckets: u128, bucket_span: u128, lowest_ns: u128) -> (Vec<String>, u16) {
    let mut to_print: Vec<String> = vec![];
    let mut largest_title: u16 = 0;

    for i in 0..nb_buckets {
        let range_start = lowest_ns + bucket_span * i;
        let range_end = range_start + bucket_span;

        let start = nano_to_hr(Duration::from_nanos(range_start as u64));
        let end = nano_to_hr(Duration::from_nanos(range_end as u64));

        let line = format!("{start} - {end}");

        if line.len() > (largest_title as usize) {
            largest_title = line.len() as u16;
        }

        to_print.push(line);
    }

    (to_print, largest_title)
}

fn compute_rest_of_line(
    time_range: Vec<String>,
    bucket: Vec<i32>,
    largest_title: u16,
    total_nb_times: f64,
    terminal_width: u16,
    largest_count_chars: u16,
    max_count: u16,
) -> Vec<String> {
    let mut to_print: Vec<String> = vec![];

    for (i, time_txt) in time_range.into_iter().enumerate() {
        let count = bucket[i];
        let pct = (count * 100) as f64 / total_nb_times;

        let nb_spaces_needed = largest_title - (time_txt.len() as u16);
        let spaces = " ".repeat(nb_spaces_needed as usize);

        let available_space_for_bar =
            terminal_width - (largest_title + largest_count_chars + LINE_DELIMITER_COUNT);

        let nb_bars = ((count as u16) * (available_space_for_bar) / max_count) as usize;
        let bar = "█".repeat(nb_bars);

        let space_before = match nb_bars > 0 {
            true => " ",
            false => "",
        };

        let line = format!("{time_txt}{spaces} │ {bar}{space_before}{pct}%");
        to_print.push(line);
    }

    to_print
}

fn compute_avg_line(
    avg_nanos: u128,
    largest_title: u16,
    largest_count_chars: u16,
    terminal_width: u16,
) -> String {
    let avg_txt = format!(
        "─── Average {}",
        nano_to_hr(Duration::from_nanos(avg_nanos.try_into().unwrap()))
    );

    let txt_len = avg_txt.len() - (3 * 2); // Because '─' takes 3 len, don't know why

    let nb_spaces_needed = largest_title - (txt_len as u16);
    let spaces = " ".repeat(nb_spaces_needed as usize);

    let available_space_for_bar =
        (terminal_width - (largest_title + largest_count_chars) + 1) as usize;

    let bar = "─".repeat(available_space_for_bar / 2);
    let line = color_txt(
        ToColorize::Str(format!("{avg_txt}{spaces}├{bar}")),
        TextColor::Yellow,
    );

    line.to_string()
}
