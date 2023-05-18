pub mod alarm {
    use chrono::Timelike;

    use crate::help::help::print_help;

    const DELIMITER: &str = ":";


    /// parse alarm string and return components (h, m, s)
    fn parse_alarm_str(s: &str) -> Option<(u32, u32, u32)> {
        let split_str = s.split(DELIMITER)
            .map(|el|el.to_owned())
            .collect::<Vec<String>>();

        if split_str.len() != 3 {
            return None;
        }

        for el in split_str.iter() {
            if el.parse::<u32>().is_err() {
                return None;
            }
        }

        let hour = split_str[0].parse::<u32>().unwrap();
        let minute = split_str[1].parse::<u32>().unwrap();
        let second = split_str[2].parse::<u32>().unwrap();

        return Some((hour, minute, second));
    }


    /// starts the alarm
    pub fn start_alarm(s: &str) {
        let parse_res = parse_alarm_str(s);
        if parse_res.is_none() {
            println!("invalid alarm string\n");
            print_help();
            return;
        }

        let (hour, minute, second) = parse_res.unwrap();
        loop {
            let now = chrono::Local::now();
            
            if hour == now.hour() && minute == now.minute() && second == now.second() {
                break;
            }
        }

        println!("done.");
    }
}