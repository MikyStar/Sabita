use crate::core::benchmark::time_utils::seconds_to_hr;

use super::{
    console_ui::{clean_last_rows, color_txt, print_table, ColoredText, TextColor, ToColorize},
    runner::{
        BenchmarkResult, FuncThreadMessage, FunctionName, ThreadLifecycleMessage,
        ThreadLifecycleMsgType, ThreadMessageType, NB_TESTS,
    },
};

use std::{sync::mpsc::Receiver, time::Instant, usize};

////////////////////////////////////////

const HEADER_SIZE: usize = 4;

////////////////////////////////////////

pub fn handle_messages(receiver: Receiver<FuncThreadMessage>, func_names: Vec<FunctionName>) {
    let mut started: Vec<u8> = vec![0; func_names.len()];
    let mut stopped: Vec<u8> = vec![0; func_names.len()];
    let mut results: Vec<Option<BenchmarkResult>> = vec![None; func_names.len()];

    let mut is_first_lifecycle = false;
    let mut has_time_printing_started = false;

    let processing_rows_len = (func_names.len() + HEADER_SIZE) as u16;

    for received in receiver {
        let FuncThreadMessage {
            func,
            msg_type,
            msg,
        } = received;

        unsafe {
            match msg_type {
                ThreadMessageType::Lifecycle => {
                    let parsed_msg: ThreadLifecycleMessage = msg.lifecycle_msg;

                    let func_index = func_names.iter().position(|&r| r == func).unwrap();

                    if !is_first_lifecycle {
                        is_first_lifecycle = true;
                    } else {
                        let to_remove = processing_rows_len + 2;

                        if has_time_printing_started {
                            clean_last_rows(to_remove);
                        } else {
                            clean_last_rows(to_remove + 1);
                        }
                    }

                    on_lifecycle(
                        parsed_msg,
                        func,
                        &mut started,
                        &mut stopped,
                        &func_names,
                        func_index,
                    );
                }
                ThreadMessageType::Result => {
                    let parsed_msg: BenchmarkResult = msg.result_msg;

                    let func_index = func_names.iter().position(|&r| r == func).unwrap();

                    on_result(
                        parsed_msg,
                        &mut results,
                        &func_names,
                        func_index,
                        processing_rows_len,
                    );

                    println!();
                }
                ThreadMessageType::Tick => {
                    let parsed_msg: Instant = msg.tick_msg;

                    let time = parsed_msg.elapsed();
                    let formatted = seconds_to_hr(time);

                    clean_last_rows(0);

                    println!(
                        "Running ... {}",
                        color_txt(ToColorize::Str(formatted), TextColor::Yellow)
                    );

                    if !has_time_printing_started {
                        has_time_printing_started = true;
                    }
                }
            }
        }
    }
}

////////////////////

fn on_lifecycle(
    message: ThreadLifecycleMessage,
    func: FunctionName,
    started: &mut Vec<u8>,
    stopped: &mut Vec<u8>,
    func_names: &Vec<FunctionName>,
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

    for (i, f_name) in func_names.clone().into_iter().enumerate() {
        let nb_started = started[i];
        let nb_stopped = stopped[i];

        let is_current_func = f_name == func;
        let is_start = lifecycle_type == ThreadLifecycleMsgType::Start;
        let is_curr_func_done = nb_stopped == NB_TESTS;

        let f_text = match is_curr_func_done {
            true => color_txt(ToColorize::FuncName(f_name), TextColor::Green),
            false => color_txt(ToColorize::FuncName(f_name), TextColor::Normal),
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
    );

    println!();
}

////////////////////

fn on_result(
    message: BenchmarkResult,
    results: &mut Vec<Option<BenchmarkResult>>,
    func_names: &Vec<FunctionName>,
    func_index: usize,
    rows_to_clean: u16,
) {
    results[func_index] = Some(message);

    let all_done = results.iter().all(|e| e.is_some());

    if all_done {
        clean_last_rows(rows_to_clean);

        let mut data: Vec<Vec<ColoredText>> = vec![];

        for (i, result) in results.clone().into_iter().enumerate() {
            match result {
                Some(val) => {
                    let BenchmarkResult {
                        slowest,
                        average,
                        fastest,
                    } = val;

                    data.push(vec![
                        color_txt(ToColorize::FuncName(func_names[i]), TextColor::Normal),
                        color_txt(ToColorize::Dur(average), TextColor::Normal),
                        color_txt(ToColorize::Dur(slowest), TextColor::Normal),
                        color_txt(ToColorize::Dur(fastest), TextColor::Normal),
                    ]);
                }
                None => panic!("Results not found"),
            }
        }

        print_table(
            vec![
                "Function".to_string(),
                "Average".to_string(),
                "Slowest".to_string(),
                "Fastest".to_string(), // TODO standard deviation
            ],
            data,
        );
    }
}
