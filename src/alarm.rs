pub mod alarm {
    use chrono::Timelike;

    const DELIMITER: &str = ":";


    /// parse alarm string and return components (h, m, s)
    fn parse_alarm_str(s: &str) -> (u32, u32, u32) {
        let split_str = s.split(DELIMITER)
            .map(|el|el.to_owned())
            .collect::<Vec<String>>();

        if split_str.len() != 3 {
            panic!("invalid alarm string");
        }

        for el in split_str.iter() {
            if el.parse::<u32>().is_err() {
                panic!("invalid alarm string");
            }
        }

        let hour = split_str[0].parse::<u32>().unwrap();
        let minute = split_str[1].parse::<u32>().unwrap();
        let second = split_str[2].parse::<u32>().unwrap();

        return (hour, minute, second);
    }


    /// starts the alarm
    pub fn start_alarm(s: &str) {
        let (hour, minute, second) = parse_alarm_str(s);

        loop {
            let now = chrono::Local::now();
            
            if hour == now.hour() && minute == now.minute() && second == now.second() {
                break;
            }
        }

        println!("done.");
    }
}