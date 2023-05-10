pub mod timer {
    use std::{collections::HashMap, time, thread, io::{self, Write}};

    // number of dots
    const N_FRACS: f64 = 40.0;
    const SYMBOL: &str = "-";
    const SYMBOL_ARROW: &str = ">";

    // unit and factor for conversion to seconds
    pub fn get_units() -> HashMap<String, f64> {
        let mut units: HashMap<String, f64> = HashMap::new(); 
        units.insert("s".to_owned(), 1.0);
        units.insert("ms".to_owned(), 0.001);
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
            thread::sleep(time::Duration::from_secs_f64(frac_time));

            let mut out_str = String::from("[");
            out_str.push_str(&SYMBOL.repeat(i-1));
            out_str.push_str(&SYMBOL_ARROW);
            out_str.push_str(&" ".repeat((N_FRACS-i as f64) as usize));
            out_str.push(']');

            clear_screen();
            let pct = i as f64*100.0/N_FRACS;
            println!("{} {}%", out_str, pct); 
            io::stdout().flush().unwrap();
        }

        println!("\ndone.");
        io::stdout().flush().unwrap();
    }

    // clear terminal screen
    fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H"); // clear screen
    }

    /// convert amount of unit to seconds
    pub fn to_seconds(amount: f64, conversion: f64) -> f64 {
        return amount * conversion;
    }

}