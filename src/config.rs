use crate::color::{Color, Stop, Theme};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct RawStop {
    color: String,
    progress: f32,
}

#[derive(Serialize, Deserialize)]
struct RawTheme {
    idle: String,
    stops: Vec<RawStop>,
}

impl TryFrom<RawTheme> for Theme {
    type Error = csscolorparser::ParseColorError;
    fn try_from(raw_theme: RawTheme) -> Result<Self, Self::Error> {
        let raw_idle = csscolorparser::parse(&raw_theme.idle)?;
        let idle_values = raw_idle.to_rgba8();
        let idle = Color {
            r: idle_values[0],
            g: idle_values[1],
            b: idle_values[2],
        };

        let stops: Vec<Stop> = raw_theme
            .stops
            .into_iter()
            .map(Stop::try_from)
            .collect::<Result<Vec<Stop>, _>>()?;

        Ok(Theme {
            idle,
            stops,
            interpolation_method: crate::color::InterpolationMethod::Lerp,
        })
    }
}

impl TryFrom<RawStop> for Stop {
    type Error = csscolorparser::ParseColorError;
    fn try_from(raw_stop: RawStop) -> Result<Self, Self::Error> {
        let raw_color = csscolorparser::parse(&raw_stop.color)?;
        let color_values = raw_color.to_rgba8();

        Ok(Stop {
            color: Color {
                r: color_values[0],
                g: color_values[1],
                b: color_values[2],
            },
            progress: raw_stop.progress,
        })
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
    let path = themes_toml.as_ref();
    let toml_as_str = fs::read_to_string(path)
        .with_context(|| format!("Reading {} as a String", path.display()))?;
    let raw_config: RawConfig = toml::from_str(&toml_as_str)?;

    let themes: HashMap<String, Theme> = raw_config
        .themes
        .into_iter()
        .map(|(name, raw_theme)| Theme::try_from(raw_theme).map(|theme| (name, theme)))
        .collect::<Result<HashMap<String, Theme>, _>>()?;
    Ok(themes)
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

        let stop_0_progress: f32 = 0.0;
        let stop_1_progress: f32 = 0.5;
        let stop_2_progress: f32 = 0.833;
        let stop_3_progress: f32 = 1.0;

        assert_eq!(themes[&theme_name].idle, correct_idle);
        assert_eq!(themes[&theme_name].stops[0].color, correct_green);
        assert_eq!(themes[&theme_name].stops[1].color, correct_yellow);
        assert_eq!(themes[&theme_name].stops[2].color, correct_red);
        assert_eq!(themes[&theme_name].stops[3].color, correct_black);

        assert_eq!(themes[&theme_name].stops[0].progress, stop_0_progress);
        assert_eq!(themes[&theme_name].stops[1].progress, stop_1_progress);
        assert_eq!(themes[&theme_name].stops[2].progress, stop_2_progress);
        assert_eq!(themes[&theme_name].stops[3].progress, stop_3_progress);
    }

    #[test]
    fn load_themes_returns_error_for_missing_file() {
        let result = load_themes("nonexistent.toml");
        assert!(result.is_err());
    }
}
