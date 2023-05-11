pub mod timer {
    use std::{collections::HashMap, time, thread, io::{self, Write}};

    // number of dots
    const N_FRACS: f64 = 40.0;
    const SYMBOL: &str = "-";
    const SYMBOL_ARROW: &str = ">";


    // unit and factor for conversion to seconds
    fn get_units() -> HashMap<String, f64> {
        let mut units: HashMap<String, f64> = HashMap::new(); 
        units.insert("s".to_owned(), 1.0);
        units.insert("m".to_owned(), 60.0);
        units.insert("h".to_owned(), 3600.0);
        units.insert("d".to_owned(), 3600.0*12.0);

        return units;
    }

    
    /// start timer with time in seconds
    pub fn start_timer(secs: f64) {
        let frac_time = secs / N_FRACS;

        clear_screen();

        for i in 1..(N_FRACS+1.0) as usize {
            let mut out_str = String::from("[");
            out_str.push_str(&SYMBOL.repeat(i-1));
            out_str.push_str(&SYMBOL_ARROW);
            out_str.push_str(&" ".repeat((N_FRACS-i as f64) as usize));
            out_str.push(']');

            clear_screen();
            let pct = i as f64*100.0/N_FRACS;
            println!("{} {}%", out_str, pct); 
            io::stdout().flush().unwrap();

            thread::sleep(time::Duration::from_secs_f64(frac_time));
        }

        println!("done.");
        io::stdout().flush().unwrap();
    }


    // clear terminal screen
    fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H"); // clear screen
    }


    /// split time string, e.g. "10m30s" -> ["10", "m", "30", "s"]
    fn parse_time_str(time_str: &str) -> Vec<String> {
        let units = get_units();

        let mut split_time = Vec::new();
        let mut word: String = String::new();
        for c in time_str.split("") {
            if units.contains_key(c) {
                split_time.push(word.clone());
                word.clear();
                split_time.push(c.to_owned());
                continue;
            }

            word.push_str(c);
        }
        if word.len() > 0 {
            split_time.push(word);
        }

        // check valid
        for el in split_time.iter() {
            if el.parse::<f64>().is_err() && !units.contains_key(el) {
                panic!("invalid time string");
            }
        }
        if split_time.len() % 2 != 0 {
            panic!("invalid time string");
        }


        return split_time;
    }


    /// convert parsed time string to seconds
    fn parsed_to_seconds(split_time: Vec<String>) -> f64 {
        let units = get_units();

        let mut seconds = 0.0;
        for i in (0..split_time.len()-1).step_by(2) {
            let amount = split_time[i].parse::<f64>().unwrap();
            seconds += amount * units[&split_time[i+1]];
        }

        return seconds;
    }


    /// convert time string to seconds
    pub fn str_to_seconds(time_str: &str) -> f64 {
        let split_time = parse_time_str(time_str);
        let seconds = parsed_to_seconds(split_time);

        return seconds;
    }

}