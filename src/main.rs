use pomo::Config;
use std::ops::{Add, Sub};
use std::time::{Duration, Instant};
use std::{env, process, thread};

fn main() {
    let configuration = build_config();
    let pomo_duration = configuration.duration();
    println!(
        "Starting pomodoro timer for {} minutes.",
        pomo_duration.as_secs() / 60
    );

    execute_pomo(pomo_duration);
    execute_break(pomo_duration);
}

fn build_config() -> Config {
    let args: Vec<String> = env::args().collect();
    let configuration = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}.", err);
        process::exit(1);
    });
    configuration
}

fn execute_pomo(pomo_duration: Duration) {
    let pomo_start = Instant::now();
    let pomo_end = pomo_start.add(pomo_duration);
    while Instant::now() < pomo_end {
        let time_remaining = pomo_end.sub(Instant::now());
        println!("Remaining: {}.", pomo::format(time_remaining));
        thread::sleep(Duration::from_secs(60));
    }
    pomo::notify("Pomodoro finished.", "Done");
}

fn execute_break(pomo_duration: Duration) {
    let break_duration = pomo_duration / 5;
    thread::sleep(break_duration);
    pomo::notify("Break finished.", "Done");
}
