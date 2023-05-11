use std::env::args;

use timer::timer::{start_timer, str_to_seconds};

pub mod timer;

fn main() {

    // syntax: temps [time]
    // [time]: 10m5s, 2h30m, ...

    let time_str = args().nth(1).expect("no time specified");
    let seconds_to_wait = str_to_seconds(time_str.trim());

    start_timer(seconds_to_wait);
}
