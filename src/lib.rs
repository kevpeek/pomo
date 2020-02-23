// Reexport items from other modules.
mod pomo_config;
pub use pomo_config::Config;

use std::process::{Command, Stdio};
use std::time::Duration;

use rodio::Sink;
use std::fs::File;
use std::io::BufReader;

pub fn notify(title: &str, message: &str) {
    let notification_command = format!(
        "display notification \"{}\" with title \"{}\"",
        title, message
    );
    Command::new("osascript")
        .arg("-e")
        .arg(notification_command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute process");

    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);
    let file = File::open("/Users/kevinpeek/Downloads/beep.ogg").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}

pub fn format(d: Duration) -> String {
    let minutes = d.as_secs() as f64 / 60.0;
    return format!("{:.0} minutes", minutes);
}

#[cfg(test)]
mod tests {
    #[test]
    fn sandbox() {}
}
