use std::{env, thread};
use std::ops::{Add, Sub};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use pomo::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let configuration = Config::new(&args).expect("Unable to parse flags");
    let pomo_duration = configuration.duration();

    println!("Starting pomodoro timer for {} minutes.", pomo_duration.as_secs() / 60);

    let start = Instant::now();
    let end = start.add(pomo_duration);

    while Instant::now() < end {
        let time_remaining = end.sub(Instant::now());
        println!("Remaining: {}.", format(time_remaining));
        thread::sleep(Duration::from_secs(60));
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