use std::time::{Instant, Duration};
use std::thread;
use std::ops::{Add, Sub};

use clap::{Arg, App};
use std::process::{Command, Stdio};

const DURATION_FLAG_NAME: &str = "duration";

fn main() {
    let matches = App::new("Pomo timer")
        .version("0.0.1")
        .author("Kevin Peek <kevpeek@gmail.com>")
        .about("A simple pomodoro timer.")
        .arg(Arg::with_name(DURATION_FLAG_NAME)
            .short("d")
            .long("duration")
            .takes_value(true)
            .default_value("25")
            .help("The duration of the pomodoro, in minutes."))
        .get_matches();

    let pomo_minutes = matches.value_of(DURATION_FLAG_NAME).unwrap();
    let pomo_minutes = pomo_minutes.parse::<u64>().expect("Invalid duration supplied");
    let pomo_duration = Duration::from_secs(pomo_minutes * 60);

    println!("Starting pomodoro timer for {} minutes.", pomo_minutes);

    let start = Instant::now();
    let end = start.add(pomo_duration);

    while Instant::now() < end {
        let time_remaining = end.sub(Instant::now());
        println!("Remaining: {}.", format(time_remaining));
        thread::sleep(pomo_duration / pomo_minutes as u32);
    }

    display_notification();
}

fn display_notification() {
    let title = "Pomodoro finished.";
    let message = "Done";
    let notification_command = format!("display notification \"{}\" with title \"{}\"", title, message);
    Command::new("osascript")
        .arg("-e")
        .arg(notification_command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute process");
}

fn format(d: Duration) -> String {
    let minutes = d.as_secs() as f64 / 60.0;
    return format!("{:.0} minutes", minutes);
}