use std::{env, thread, process};
use std::ops::{Add, Sub};
use std::time::{Duration, Instant};
use pomo::{Config};




fn main() {
    let args: Vec<String> = env::args().collect();
    let configuration = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}.", err);
        process::exit(1);
    });
    let pomo_duration = configuration.duration();

    println!("Starting pomodoro timer for {} minutes.", pomo_duration.as_secs() / 60);

    let start = Instant::now();
    let end = start.add(pomo_duration);

    while Instant::now() < end {
        let time_remaining = end.sub(Instant::now());
        println!("Remaining: {}.", pomo::format(time_remaining));
        thread::sleep(Duration::from_secs(60));
    }

    pomo::notify();
}
