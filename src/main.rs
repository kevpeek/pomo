use std::time::{Instant, Duration};
use std::thread;
use std::ops::{Add, Sub};

use notify_rust::Notification;

const POMO_LENGTH: Duration = Duration::from_secs(25 * 60);

fn main() {
    println!("Starting...");

    let start = Instant::now();
    let end = start.add(POMO_LENGTH);

    while Instant::now() < end {
        let remaining = end.sub(Instant::now());
        println!("Remaining: {}.", format(remaining));
        thread::sleep(POMO_LENGTH / 25);
    }

    Notification::new()
        .summary("Done")
        .body("Pomodoro complete")
        .show()
        .unwrap();
}

fn format(d: Duration) -> String {
    let minutes = (d.as_secs() / 60).to_string();
    let seconds = (d.as_secs() % 60).to_string();
    return format!("{} minutes, {} seconds", minutes, seconds);
}