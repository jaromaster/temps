pub mod timer {
    use std::{collections::HashMap, time, thread, io::{self, Write}};

    use crossterm::{ExecutableCommand, terminal::{Clear, ClearType}, cursor::MoveUp};

    // progress bar
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

        println!("");

        // print progress bar
        for i in 1..(N_FRACS+1.0) as usize {
            clear_screen();
            let pct = i as f64*100.0/N_FRACS;
            let out_str = format!("[{}{}{}] {}%", 
                SYMBOL.repeat(i-1),
                SYMBOL_ARROW,
                " ".repeat((N_FRACS-i as f64) as usize),
                pct
            );
            
            println!("{}", out_str); 
            io::stdout().flush().unwrap();
            thread::sleep(time::Duration::from_secs_f64(frac_time));
        }

        println!("done.");
        io::stdout().flush().unwrap();
    }


    // clear terminal screen (current line)
    fn clear_screen() {
        io::stdout().execute(MoveUp(1)).unwrap();
        io::stdout().execute(Clear(ClearType::FromCursorDown)).unwrap();
    }


    /// split time string, e.g. "10m30s" -> ["10", "m", "30", "s"]
    fn parse_time_str(time_str: &str) -> Option<Vec<String>> {
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
                return None;
            }
        }
        if split_time.len() % 2 != 0 {
            return None;
        }


        return Some(split_time);
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
    pub fn str_to_seconds(time_str: &str) -> Option<f64> {
        let split_time_res = parse_time_str(time_str);
        if split_time_res.is_none() {
           return None

        }

        let split_time = split_time_res.unwrap();
        let seconds = parsed_to_seconds(split_time);

        return Some(seconds);
    }
}