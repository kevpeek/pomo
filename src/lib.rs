use clap::{App, Arg};

const DURATION_FLAG_NAME: &str = "duration";
const DEFAULT_DURATION: &str = "25";

pub struct Config {
    pub pomo_minutes: u64
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let matches = App::new("Pomo timer")
            .version("0.0.1")
            .author("Kevin Peek <kevpeek@gmail.com>")
            .about("A simple pomodoro timer.")
            .arg(Arg::with_name(DURATION_FLAG_NAME)
                .short("d")
                .long("duration")
                .takes_value(true)
                .default_value(DEFAULT_DURATION)
                .help("The duration of the pomodoro, in minutes."))
            .get_matches_from(args);
        let pomo_minutes = matches.value_of(DURATION_FLAG_NAME).unwrap();
        let pomo_minutes = pomo_minutes.parse::<u64>().expect("Invalid duration supplied");
        let configuration = Config { pomo_minutes };
        configuration
    }
}



#[cfg(test)]
mod tests {
    use crate::{Config};

    #[test]
    fn default_params() {
        let args = [String::from("pomo")];
        let config = Config::new(&args);
        assert_eq!(25, config.pomo_minutes);
    }

}