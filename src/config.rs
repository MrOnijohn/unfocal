use crate::color::{Color, Theme};
use anyhow::Error;
use csscolorparser::parse;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
struct RawStop {
    position: f32,
    color: String,
}

#[derive(Serialize, Deserialize)]
struct RawTheme {
    idle: String,
    stops: Vec<RawStop>,
}

impl TryFrom<RawTheme> for Theme {
    type Error = anyhow::Error;
    fn try_from(value: RawTheme) -> Result<Self, Self::Error> {
        todo!();
    }
}

#[derive(Serialize, Deserialize)]
struct RawConfig {
    themes: HashMap<String, RawTheme>,
}

struct Settings {
    focus_time: u32,
    selected_theme: String,
}

pub struct Config {
    themes: HashMap<String, Theme>,
    settings: Settings,
}

pub fn load_themes(themes_toml: impl AsRef<Path>) -> Result<HashMap<String, Theme>, anyhow::Error> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_themes_returns_valid_default_theme() {
        let test_file = "themes_test.toml";
        let theme_name = "default".to_string();
        let themes = load_themes(test_file).unwrap();

        let correct_idle = Color {
            r: 0,
            g: 204,
            b: 204,
        };
        let correct_green = Color { r: 0, g: 204, b: 0 };
        let correct_yellow = Color {
            r: 204,
            g: 204,
            b: 0,
        };
        let correct_red = Color { r: 204, g: 0, b: 0 };
        let correct_black = Color { r: 0, g: 0, b: 0 };

        let stop_0: f32 = 0.0;
        let stop_1: f32 = 0.5;
        let stop_2: f32 = 0.833;
        let stop_3: f32 = 1.0;

        assert_eq!(themes[theme_name].idle, correct_idle);
        assert_eq!(themes[theme_name].stops[0], correct_green);
        assert_eq!(themes[theme_name].stops[1], correct_yellow);
        assert_eq!(themes[theme_name].stops[2], correct_red);
        assert_eq!(themes[theme_name].stops[3], correct_black);
    }
}
