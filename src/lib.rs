// Reexport items from other modules.
mod pomo_config;
pub use pomo_config::Config;

use std::process::{Command, Stdio};
use std::time::Duration;

use rodio::Sink;
use std::env;
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

    let sound_file_path = pomo_sound_path().unwrap();
    let file = File::open(sound_file_path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);
    sink.append(source);
    sink.sleep_until_end();
}

fn pomo_sound_path() -> Option<String> {
    let sound_file_path = env::var("POMO_SOUND").unwrap();
    Some(sound_file_path)
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
