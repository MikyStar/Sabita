use std::time::Duration;

use humanize_duration::{prelude::DurationExt, Truncate};

////////////////////////////////////////

pub fn nano_to_hr(time: Duration) -> String {
    time.human(Truncate::Nano).to_string()
}

pub fn seconds_to_hr(time: Duration) -> String {
    time.human(Truncate::Second).to_string()
}
