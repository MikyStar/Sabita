use std::{cmp::max, time::Duration};

use super::{
    console_ui::{color_txt, get_terminal_width, TextColor, ToColorize},
    time_utils::nano_to_hr,
};
use crate::core::benchmark::runner::BenchmarkResult;

////////////////////////////////////////

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
    let largest_count_chars = max_count.to_string().len() as u16;
    let static_delimiter = 4; // Counting spaces and |

    let (to_print_before, largest_title_before) = compute_time_range(
        before_bucket_size as u128,
        nb_buckets_arround as usize,
        min_nanos,
    );
    let (to_print_after, largest_title_after) = compute_time_range(
        before_bucket_size as u128,
        nb_buckets_arround as usize,
        min_nanos,
    );
    let largest_title: u16 = max(largest_title_before, largest_title_after);

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

////////////////////

fn compute_time_range(nb_buckets: u128, bucket_size: usize, lowest_ns: u128) -> (Vec<String>, u16) {
    let mut to_print: Vec<String> = vec![];
    let mut largest_title: u16 = 0;

    for i in 0..bucket_size {
        let range_start = lowest_ns + nb_buckets * (i as u128);
        let range_end = range_start + nb_buckets;

        let start = nano_to_hr(Duration::from_nanos(range_start.try_into().unwrap()));
        let end = nano_to_hr(Duration::from_nanos(range_end.try_into().unwrap()));

        let line = format!("{start} - {end}");

        if line.len() > (largest_title as usize) {
            largest_title = line.len() as u16;
        }

        to_print.push(line);
    }

    (to_print, largest_title)
}
