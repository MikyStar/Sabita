use super::{
    console_ui::{
        clear_lines_from, color_txt, get_cursor_position, print_table, queue_msg, ColoredText,
        CursorPos, TextColor, ToColorize,
    },
    file::write,
    histogram::draw_histogram,
    runner::{
        BenchmarkResult, FuncThreadMessage, ThreadLifecycleMessage, ThreadLifecycleMsgType,
        ThreadMessageType, NB_TESTS,
    },
    time_utils::seconds_to_hr,
};

use std::{
    mem::ManuallyDrop,
    process::exit,
    sync::mpsc::Receiver,
    time::{Duration, Instant},
};

////////////////////////////////////////

pub fn handle_messages(
    receiver: Receiver<FuncThreadMessage>,
    func_names: Vec<String>,
    start: Instant,
    nb_buckets_arround: u128,
    file_path: Option<String>,
) {
    let mut started: Vec<u8> = vec![0; func_names.len()];
    let mut stopped: Vec<u8> = vec![0; func_names.len()];
    let mut results: Vec<Option<BenchmarkResult>> = vec![None; func_names.len()];

    let mut is_first_lifecycle = false;

    let base_cursor_pos = get_cursor_position();
    let mut after_table_cursor_pos: Option<CursorPos> = None;

    for received in receiver {
        let FuncThreadMessage {
            func,
            msg_type,
            msg,
        } = received;

        unsafe {
            match msg_type {
                ThreadMessageType::Lifecycle => {
                    let mut _parsed_msg: ManuallyDrop<ThreadLifecycleMessage> = msg.lifecycle_msg;
                    let parsed_msg = ManuallyDrop::into_inner(_parsed_msg);

                    let func_index = func_names.iter().position(|r| *r == func).unwrap();

                    if !is_first_lifecycle {
                        is_first_lifecycle = true;
                    } else {
                        clear_lines_from(base_cursor_pos);
                    }

                    handle_progress(
                        parsed_msg,
                        func,
                        &mut started,
                        &mut stopped,
                        &func_names,
                        func_index,
                    );

                    if after_table_cursor_pos.is_none() {
                        after_table_cursor_pos = Some(get_cursor_position());
                    }

                    handle_clock(start, after_table_cursor_pos.unwrap());
                }
                ThreadMessageType::Result => {
                    let mut _parsed_msg: ManuallyDrop<BenchmarkResult> = msg.result_msg;
                    let parsed_msg = ManuallyDrop::into_inner(_parsed_msg);

                    let func_index = func_names.iter().position(|r| *r == func).unwrap();

                    results[func_index] = Some(parsed_msg);

                    let all_done = results.iter().all(|e| e.is_some());

                    if all_done {
                        clear_lines_from(base_cursor_pos);
                        print_table_results(results.clone(), &func_names, file_path.clone());
                        print_histograms_results(
                            results.clone(),
                            &func_names,
                            nb_buckets_arround,
                            file_path,
                        );

                        exit(0);
                    }
                }
                ThreadMessageType::Tick => {
                    if let Some(pos) = after_table_cursor_pos {
                        handle_clock(start, pos);
                    }
                }
            }
        }
    }
}

////////////////////

fn handle_progress(
    message: ThreadLifecycleMessage,
    func: String,
    started: &mut [u8],
    stopped: &mut [u8],
    func_names: &[String],
    func_index: usize,
) {
    let ThreadLifecycleMessage {
        msg_type: lifecycle_type,
        ..
    } = message;

    match lifecycle_type {
        ThreadLifecycleMsgType::Start => started[func_index] += 1,
        ThreadLifecycleMsgType::Stop => stopped[func_index] += 1,
    }

    let mut data: Vec<Vec<ColoredText>> = vec![];

    for (i, f_name) in func_names.iter().enumerate() {
        let nb_started = started[i];
        let nb_stopped = stopped[i];

        let is_current_func = *f_name == func;
        let is_start = lifecycle_type == ThreadLifecycleMsgType::Start;
        let is_curr_func_done = nb_stopped == NB_TESTS;

        let f_text = match is_curr_func_done {
            true => color_txt(ToColorize::Str(f_name.to_string()), TextColor::Green),
            false => color_txt(ToColorize::Str(f_name.to_string()), TextColor::Normal),
        };

        let started_txt = match is_start & is_current_func & !is_curr_func_done {
            true => color_txt(ToColorize::U8(nb_started), TextColor::Yellow),
            false => color_txt(ToColorize::U8(nb_started), TextColor::Normal),
        };

        let ended_txt = match !is_start & is_current_func & !is_curr_func_done {
            true => color_txt(ToColorize::U8(nb_stopped), TextColor::Yellow),
            false => color_txt(ToColorize::U8(nb_stopped), TextColor::Normal),
        };

        data.push(vec![f_text, started_txt, ended_txt]);
    }

    print_table(
        vec![
            "Function".to_string(),
            "Started".to_string(),
            "Done".to_string(),
        ],
        data,
        None,
    );
}

////////////////////

fn print_table_results(
    results: Vec<Option<BenchmarkResult>>,
    func_names: &[String],
    file_path: Option<String>,
) {
    let mut data: Vec<Vec<ColoredText>> = vec![];

    for (i, result) in results.iter().enumerate() {
        match result {
            None => panic!("Results not found"),
            Some(val) => {
                let BenchmarkResult {
                    slowest,
                    average,
                    fastest,
                    std_dev,
                    ..
                } = val;

                data.push(vec![
                    color_txt(ToColorize::Str(func_names[i].to_string()), TextColor::Cyan),
                    color_txt(ToColorize::Dur(*average), TextColor::Yellow),
                    color_txt(ToColorize::Dur(*slowest), TextColor::Normal),
                    color_txt(ToColorize::Dur(*fastest), TextColor::Normal),
                    color_txt(ToColorize::Dur(*std_dev), TextColor::Normal),
                ]);
            }
        }
    }

    print_table(
        vec![
            color_txt(ToColorize::Str("Function".to_string()), TextColor::Cyan).to_string(),
            color_txt(ToColorize::Str("Average".to_string()), TextColor::Yellow).to_string(),
            "Slowest".to_string(),
            "Fastest".to_string(),
            "Std dev".to_string(),
        ],
        data,
        file_path,
    );
}

fn print_histograms_results(
    results: Vec<Option<BenchmarkResult>>,
    func_names: &[String],
    nb_buckets_arround: u128,
    file_path: Option<String>,
) {
    for (i, result) in results.iter().enumerate() {
        match result {
            None => panic!("Results not found"),
            Some(res) => {
                let f_name = &func_names[i];

                let txt =
                    color_txt(ToColorize::Str(f_name.to_string()), TextColor::Cyan).to_string();

                queue_msg(txt.clone());
                if let Some(ref path) = file_path {
                    write(path.to_string(), vec![txt, "".to_string()]);
                }

                println!();
                draw_histogram(res, nb_buckets_arround, file_path.clone());

                if i != results.len() - 1 {
                    println!();
                    if let Some(ref path) = file_path {
                        write(path.to_string(), vec!["".to_string()]);
                    }
                }
            }
        }
    }
}

////////////////////

fn handle_clock(start: Instant, after_table_cursor_pos: CursorPos) {
    let time = start.elapsed();
    let wait_a_bit = Duration::from_secs(5);

    if time > wait_a_bit {
        clear_lines_from(after_table_cursor_pos);

        let formatted = seconds_to_hr(time);

        queue_msg(format!(
            "Running ... {}",
            color_txt(ToColorize::Str(formatted), TextColor::Yellow)
        ));
    }
}
