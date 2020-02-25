use indicatif::ProgressBar;
use pomo::Config;
use std::ops::Add;
use std::time::{Duration, Instant};
use std::{env, process, thread};

fn main() {
    let configuration = build_config();
    let pomo_duration = configuration.duration();

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
    println!(
        "Starting pomodoro timer for {} minutes.",
        pomo_duration.as_secs() / 60
    );

    let pomo_start = Instant::now();
    let pomo_end = pomo_start.add(pomo_duration);

    let bar = ProgressBar::new(100);
    bar.tick(); // force the bar to show

    while Instant::now() < pomo_end {
        thread::sleep(pomo_duration / 100);
        bar.inc(1);
    }
    bar.finish();
    pomo::notify("Pomodoro finished.", "Done");
}

fn execute_break(pomo_duration: Duration) {
    let break_duration = pomo_duration / 5;

    println!(
        "Taking a break for {} minutes.",
        break_duration.as_secs() / 60
    );

    let break_start = Instant::now();
    let break_end = break_start.add(break_duration);

    let bar = ProgressBar::new(100);
    bar.tick(); // force the bar to show

    while Instant::now() < break_end {
        thread::sleep(break_duration / 100);
        bar.inc(1);
    }
    bar.finish();
    pomo::notify("Break finished.", "Done");
}
