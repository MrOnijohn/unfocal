use crate::color::Theme;
use directories::ProjectDirs;
use eframe::egui::ahash::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
struct RawTheme {
    raw_stops: Vec<String>,
    raw_interpolation_method: String,
}

impl From<RawTheme> for Theme {
    fn from(value: RawTheme) -> Self {
        todo!();
    }
}

struct Settings {
    focus_time: Duration,
    theme: Theme,
}

pub struct Config {
    themes: HashMap<String, Theme>,
    settings: Settings,
}

pub fn load_themes(themes_toml: impl AsRef<Path>) -> HashMap<String, Theme> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_themes_returns_valid_themes() {
        todo!();
    }
}
