use std::process::{Command, Stdio};
use std::time::Duration;

use rodio::{Sink, Source};
use std::env;
use std::fs::File;
use std::io::BufReader;

use dialoguer::Confirmation;

// notify alerts the user via a desktop notification and audible alert.
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

    match pomo_sound_path() {
        Some(path) => play_alert_from_file(path),
        None => play_default_alert(),
    };
}

fn play_alert_from_file(path: String) {
    let file = File::open(path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    sink.append(source.repeat_infinite());

    // play the sound until the user acknowledges the alert.
    Confirmation::new()
        .with_text("Press enter to continue")
        .show_default(false)
        .interact()
        .unwrap();
    sink.stop();
}

fn play_default_alert() {
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);
    let source = rodio::source::SineWave::new(440).take_duration(Duration::from_secs(4));
    sink.append(source);
    sink.sleep_until_end();
}

// pomo_sound_path tries to read the sound file path from the ENV.
fn pomo_sound_path() -> Option<String> {
    match env::var("POMO_SOUND") {
        Ok(path) => Some(path),
        Err(_) => None,
    }
}
