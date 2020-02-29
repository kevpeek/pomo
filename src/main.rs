use indicatif::{ProgressBar, ProgressStyle};
use pomo::Config;
use std::ops::Add;
use std::time::{Duration, Instant};
use std::{env, process, thread};

const POMO_BAR_STYLE: &str = "{wide_bar:50.cyan/yellow} {eta}";
const BREAK_BAR_STYLE: &str = "{wide_bar:50.green/yellow} {eta}";

fn main() {
    let configuration = build_config();
    execute_pomo(configuration.duration());
    execute_break(configuration.duration());
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

    wait_with_progress_bar(pomo_duration, POMO_BAR_STYLE);
    pomo::notify("Pomodoro finished.", "Done");
}

fn execute_break(pomo_duration: Duration) {
    let break_duration = pomo_duration / 5;

    println!(
        "Taking a break for {} minutes.",
        break_duration.as_secs() / 60
    );

    wait_with_progress_bar(break_duration, BREAK_BAR_STYLE);
    pomo::notify("Break finished.", "Done");
}

// wait_with_progress_bar causes the thread to sleep for the specified duration while
// updating a progress bar to reflect the progress of the wait.
fn wait_with_progress_bar(duration: Duration, style: &str) {
    let start = Instant::now();
    let end = start.add(duration);
    let bar = ProgressBar::new(100);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(style)
            .progress_chars("#>-"),
    );
    bar.tick(); // force the bar to show
    while Instant::now() < end {
        thread::sleep(duration / 100);
        bar.inc(1);
    }
    bar.finish();
}
