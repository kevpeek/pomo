use clap::{App, Arg};
use std::time::Duration;

const DURATION_FLAG_NAME: &str = "duration";
const DEFAULT_DURATION: &str = "25";

pub struct Config {
    pomo_minutes: u64,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let matches = App::new("Pomo timer")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Kevin Peek <kevpeek@gmail.com>")
            .about("A simple pomodoro timer.")
            .arg(Config::duration_arg())
            .get_matches_from(args);

        // duration arg has a default value, so this will be Some()
        let pomo_minutes = matches.value_of(DURATION_FLAG_NAME).unwrap();
        let pomo_minutes = match pomo_minutes.parse::<u64>() {
            Ok(v) => v,
            Err(_) => return Err("duration must be a valid number"),
        };

        Ok(Config { pomo_minutes })
    }

    // duration_arg returns an clap::Arg for the --duration flag.
    fn duration_arg<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name(DURATION_FLAG_NAME)
            .short("d")
            .long("duration")
            .takes_value(true)
            .default_value(DEFAULT_DURATION)
            .help("The duration of the pomodoro, in minutes.")
    }

    #[must_use]
    pub fn duration(self: &Config) -> Duration {
        Duration::from_secs(self.pomo_minutes * 60)
    }
}

#[cfg(test)]
mod tests {
    use crate::pomo_config::Config;
    use std::time::Duration;

    #[test]
    fn config_new_default_params() {
        let args = [String::from("pomo")];
        let config = Config::new(&args).unwrap();
        assert_eq!(25, config.pomo_minutes);
    }

    #[test]
    fn config_new_parses_short_flag() {
        let args = [String::from("pomo"), String::from("-d"), String::from("11")];
        let config = Config::new(&args).unwrap();
        assert_eq!(11, config.pomo_minutes);
    }

    #[test]
    fn config_duration() {
        let config = Config { pomo_minutes: 11 };
        let duration = config.duration();
        assert_eq!(Duration::from_secs(11 * 60), duration);
    }
}
