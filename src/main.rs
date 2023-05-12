use std::env::args;

use alarm::alarm::start_alarm;
use timer::timer::{start_timer, str_to_seconds};

pub mod timer;
pub mod alarm;

fn main() {

    // syntax: temps [mode] [time]
    // e.g. temps timer 20m10s
    // e.g. temps alarm 20:30:00

    let mode = args().nth(1).expect("no mode specified");
    let time_str = args().nth(2).expect("no time specified");

    if mode == "timer" {
        let seconds_to_wait = str_to_seconds(time_str.trim());
        start_timer(seconds_to_wait);
    }
    else if mode == "alarm" {
        start_alarm(&time_str);
    }
    else {
        println!("invalid mode '{}'", mode);
    }   
}
