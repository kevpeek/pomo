// Reexport items from other modules.
mod notify;
mod pomo_config;
pub use notify::notify;
pub use pomo_config::Config;

use std::time::Duration;

pub fn format(d: Duration) -> String {
    let minutes = d.as_secs() as f64 / 60.0;
    return format!("{:.0} minutes", minutes);
}

#[cfg(test)]
mod tests {
    #[test]
    fn sandbox() {}
}
