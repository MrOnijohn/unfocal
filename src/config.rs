use crate::color::{Color, Stop, Theme};
use anyhow::Context;
use log::warn;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub const DEFAULT_THEMES: &str = include_str!("../assets/themes.toml");

#[derive(Serialize, Deserialize)]
struct RawStop {
    color: String,
    progress: f32,
}

#[derive(Deserialize)]
struct RawTheme {
    idle: String,
    #[serde(default)]
    clock_bg: Option<String>,
    #[serde(default)]
    clock_digits: Option<String>,
    stops: Vec<RawStop>,
}

impl TryFrom<RawTheme> for Theme {
    type Error = csscolorparser::ParseColorError;
    fn try_from(raw_theme: RawTheme) -> Result<Self, Self::Error> {
        let idle: Color = raw_theme.idle.parse()?;

        let clock_bg: Color = raw_theme
            .clock_bg
            .map(|s| s.parse())
            .transpose()?
            .unwrap_or(Color::BLACK);

        let clock_digits: Color = raw_theme
            .clock_digits
            .map(|s| s.parse())
            .transpose()?
            .unwrap_or(Color::WHITE);

        let stops: Vec<Stop> = raw_theme
            .stops
            .into_iter()
            .map(Stop::try_from)
            .collect::<Result<Vec<Stop>, _>>()?;

        Ok(Theme {
            idle,
            clock_bg,
            clock_digits,
            stops,
            interpolation_method: crate::color::InterpolationMethod::Lerp,
        })
    }
}

impl TryFrom<RawStop> for Stop {
    type Error = csscolorparser::ParseColorError;
    fn try_from(raw_stop: RawStop) -> Result<Self, Self::Error> {
        let color: Color = raw_stop.color.parse()?;

        Ok(Stop {
            color,
            progress: raw_stop.progress,
        })
    }
}

#[derive(Deserialize)]
struct RawConfig {
    themes: HashMap<String, RawTheme>,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub show_settings: bool,
    pub focus_time: u32,
    pub selected_theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_settings: true,
            focus_time: 30,
            selected_theme: "default".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct Config {
    pub themes: HashMap<String, Theme>,
    pub settings: Settings,
}

impl Config {
    pub fn new(themes: HashMap<String, Theme>, settings: Settings) -> Self {
        Self { themes, settings }
    }
}

impl Default for Config {
    fn default() -> Self {
        let themes: HashMap<String, Theme> =
            toml::from_str(DEFAULT_THEMES).expect("assets/themes.toml malformed");
        let settings = Settings::default();
        Self { themes, settings }
    }
}

pub fn load_themes(themes_toml: impl AsRef<Path>) -> HashMap<String, Theme> {
    match try_load_themes(themes_toml) {
        Ok(themes) => themes,
        Err(e) => {
            warn!("Failed to load themes: {e}");
            warn!("Falling back to defaults");
            let themes: HashMap<String, Theme> =
                toml::from_str(DEFAULT_THEMES).expect("assets/themes.toml malformed");
            themes
        }
    }
}

fn try_load_themes(themes_toml: impl AsRef<Path>) -> Result<HashMap<String, Theme>, anyhow::Error> {
    let toml_as_str = get_toml_as_str(themes_toml).context("Getting toml data from themes.toml")?;
    let raw_config: RawConfig =
        toml::from_str(&toml_as_str).with_context(|| format!("Parsing toml: {}", &toml_as_str))?;

    let themes: HashMap<String, Theme> = raw_config
        .themes
        .into_iter()
        .map(|(name, raw_theme)| Theme::try_from(raw_theme).map(|theme| (name, theme)))
        .collect::<Result<HashMap<String, Theme>, _>>()
        .context("Parsing hex rgb")?;
    Ok(themes)
}

pub fn load_settings(settings_toml: impl AsRef<Path>) -> Settings {
    // TODO Check if settings.toml exists and don't warn, but inform
    match try_load_settings(settings_toml) {
        Ok(settings) => settings,
        Err(e) => {
            warn!("Failed to load settings {e}");
            warn!("Falling back to defaults");
            Settings::default()
        }
    }
}

fn try_load_settings(settings_toml: impl AsRef<Path>) -> Result<Settings, anyhow::Error> {
    let toml_as_str =
        get_toml_as_str(settings_toml).context("Loading settings from settings.toml")?;
    let settings: Settings = toml::from_str(&toml_as_str).context("Parsing settings.toml")?;

    Ok(settings)
}

fn get_toml_as_str(path: impl AsRef<Path>) -> Result<String, anyhow::Error> {
    let path = path.as_ref();
    let toml_as_str = fs::read_to_string(path)
        .with_context(|| format!("Reading {} as a String", path.display()))?;

    Ok(toml_as_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_themes_returns_valid_default_theme() {
        let test_file = "themes_test.toml";
        let theme_name = "default".to_string();
        let themes = load_themes(test_file);

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

    #[cfg(test)]
    fn valid_raw_theme() -> RawTheme {
        RawTheme {
            idle: "#00cccc".into(),
            stops: vec![
                RawStop {
                    progress: 0.0,
                    color: "#00cc00".into(),
                },
                RawStop {
                    progress: 1.0,
                    color: "#000000".into(),
                },
            ],
            clock_bg: Some("#000000".into()),
            clock_digits: Some("#ffffff".into()),
        }
    }

    #[test]
    fn missing_clock_bg_returns_black() {
        let raw = RawTheme {
            clock_bg: None,
            ..valid_raw_theme()
        };
        let theme = Theme::try_from(raw).unwrap();

        assert_eq!(theme.clock_bg, Color::BLACK);
    }

    #[test]
    fn missing_clock_digits_returns_white() {
        let raw = RawTheme {
            clock_digits: None,
            ..valid_raw_theme()
        };
        let theme = Theme::try_from(raw).unwrap();

        assert_eq!(theme.clock_digits, Color::WHITE);
    }

    #[test]
    fn malformed_clock_bg_errors() {
        let raw = RawTheme {
            clock_bg: Some("#0000000".into()),
            ..valid_raw_theme()
        };

        assert!(Theme::try_from(raw).is_err());
    }

    #[test]
    fn malformed_clock_digits_errors() {
        let raw = RawTheme {
            clock_digits: Some("#fffffff".into()),
            ..valid_raw_theme()
        };

        assert!(Theme::try_from(raw).is_err());
    }
}
