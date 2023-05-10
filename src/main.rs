use std::env::args;

use timer::timer::{get_units, to_seconds, start_timer};

pub mod timer;

fn main() {

    let units = get_units();

    // syntax: temps [n] [unit]
    let amount = args().nth(1).expect("no amount specified.");
    let unit = args().nth(2).expect("no unit specified.");

    let amount_num: f64 = amount.parse().expect("could not convert amount to float.");


    if !units.contains_key(&unit) {
        println!("invalid unit '{}'.", &unit);
    }

    let seconds_to_wait = to_seconds(amount_num, units[&unit]);
    start_timer(seconds_to_wait);
}
