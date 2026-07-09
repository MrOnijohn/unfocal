use crate::color::{Color, Stop, Theme};
use anyhow::Context;
use log::warn;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

pub const DEFAULT_THEMES: &str = include_str!("../assets/themes.toml");

enum Message {
    ErrorMessage {},
}

impl Message {
    fn message_to_string(self) -> String {
        todo!();
    }
}

#[derive(Serialize, Deserialize)]
struct RawStop {
    color: String,
    progress: f32,
}

#[derive(Error, Debug)]
pub enum ParseStopError {
    #[error("Invalid progress value {value}")]
    InvalidProgressValue { value: f32 },
    #[error("Invalid hex value {hex_string}")]
    InvalidHexValue {
        hex_string: String,
        #[source]
        parse_color_error: csscolorparser::ParseColorError,
    },
}

impl TryFrom<RawStop> for Stop {
    type Error = ParseStopError;
    fn try_from(raw_stop: RawStop) -> Result<Self, Self::Error> {
        let color: Color =
            raw_stop
                .color
                .parse()
                .map_err(|cause| ParseStopError::InvalidHexValue {
                    hex_string: raw_stop.color,
                    parse_color_error: cause,
                })?;
        if !(0.0..=1.0).contains(&raw_stop.progress) {
            return Err(ParseStopError::InvalidProgressValue {
                value: raw_stop.progress,
            });
        }
        Ok(Stop {
            color,
            progress: raw_stop.progress,
        })
    }
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

#[derive(Error, Debug)]
pub enum ParseThemeError {
    #[error("Invalid color value for idle: {idle_hex}")]
    InvalidIdleValue {
        idle_hex: String,
        #[source]
        parse_color_error: csscolorparser::ParseColorError,
    },
    #[error("Invalid color value for clock background: {clock_bg_hex}")]
    InvalidClockBG {
        clock_bg_hex: String,
        #[source]
        parse_color_error: csscolorparser::ParseColorError,
    },
    #[error("Invalid color value for clock digits: {clock_digits_hex}")]
    InvalidClockDigits {
        clock_digits_hex: String,
        #[source]
        parse_color_error: csscolorparser::ParseColorError,
    },
    #[error("Invalid stop")]
    InvalidStop(#[from] ParseStopError),
}

impl TryFrom<RawTheme> for Theme {
    type Error = ParseThemeError;
    fn try_from(raw_theme: RawTheme) -> Result<Self, Self::Error> {
        let idle: Color =
            raw_theme
                .idle
                .parse()
                .map_err(|cause| ParseThemeError::InvalidIdleValue {
                    idle_hex: raw_theme.idle,
                    parse_color_error: cause,
                })?;

        let clock_bg: Color = raw_theme
            .clock_bg
            .map(|s| {
                s.parse().map_err(|cause| ParseThemeError::InvalidClockBG {
                    clock_bg_hex: s,
                    parse_color_error: cause,
                })
            })
            .transpose()?
            .unwrap_or(Color::BLACK);

        let clock_digits: Color = raw_theme
            .clock_digits
            .map(|s| {
                s.parse()
                    .map_err(|cause| ParseThemeError::InvalidClockDigits {
                        clock_digits_hex: s,
                        parse_color_error: cause,
                    })
            })
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

#[derive(Deserialize)]
struct RawConfig {
    themes: HashMap<String, RawTheme>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum ShowClock {
    Never,
    Always,
    OnMouseOver,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub show_settings: bool,
    pub show_clock: ShowClock,
    pub focus_time: u32,
    pub selected_theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_settings: true,
            show_clock: ShowClock::OnMouseOver,
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

#[derive(Error, Debug)]
pub enum LoadThemesError {
    #[error("Failed to read themes.toml")]
    ReadTomlFailed(#[from] std::io::Error),
    #[error("Failed to parse themes.toml")]
    ParseTomlFailed(#[from] toml::de::Error),
    #[error("Failed to parse {name}")]
    InvalidTheme {
        name: String,
        #[source]
        parse_theme_error: ParseThemeError,
    },
}

fn try_load_themes(
    themes_toml: impl AsRef<Path>,
) -> (HashMap<String, Theme>, Vec<LoadThemesError>) {
    let toml_contents = match get_toml_as_str(themes_toml) {
        Ok(toml_contents) => toml_contents,
        Err(e) => return (HashMap::new(), vec![LoadThemesError::ReadTomlFailed(e)]),
    };

    let raw_config: RawConfig = match toml::from_str(&toml_contents) {
        Ok(raw_config) => raw_config,
        Err(e) => return (HashMap::new(), vec![LoadThemesError::ParseTomlFailed(e)]),
    };

    let mut themes: HashMap<String, Theme> = HashMap::new();
    let mut errors: Vec<LoadThemesError> = Vec::new();
    for (name, raw_theme) in raw_config.themes.into_iter() {
        match Theme::try_from(raw_theme) {
            Ok(theme) => {
                themes.insert(name, theme);
            }
            Err(e) => errors.push(LoadThemesError::InvalidTheme {
                name,
                parse_theme_error: e,
            }),
        };
    }

    (themes, errors)
}

pub fn load_themes(
    themes_toml: impl AsRef<Path>,
) -> (HashMap<String, Theme>, Vec<LoadThemesError>) {
    let (themes, errors) = try_load_themes(&themes_toml);
    if !themes.is_empty() {
        (themes, errors)
    } else {
        warn!(
            "Failed to load themes from {}",
            themes_toml.as_ref().display()
        );
        warn!("Falling back to defaults");
        let themes: HashMap<String, Theme> =
            toml::from_str(DEFAULT_THEMES).expect("assets/themes.toml malformed");
        (themes, errors)
    }
}

pub fn load_settings(settings_toml: impl AsRef<Path>) -> Settings {
    // TODO: Check if settings.toml exists and don't warn, but inform
    // TODO: Check if current_theme is in themes
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

fn get_toml_as_str(path: impl AsRef<Path>) -> Result<String, std::io::Error> {
    let path = path.as_ref();
    fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_themes_returns_valid_default_theme() {
        let test_file = "themes_test.toml";
        let theme_name = "default".to_string();
        let (themes, _) = load_themes(test_file);

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
    fn malformed_clock_bg_produces_correct_error_variant() {
        let raw = RawTheme {
            clock_bg: Some("#0000000".into()),
            ..valid_raw_theme()
        };

        assert!(matches!(
            Theme::try_from(raw),
            Err(ParseThemeError::InvalidClockBG { .. })
        ));
    }

    #[test]
    fn malformed_clock_digits_produces_correct_error_variant() {
        let raw = RawTheme {
            clock_digits: Some("#fffffff".into()),
            ..valid_raw_theme()
        };

        assert!(matches!(
            Theme::try_from(raw),
            Err(ParseThemeError::InvalidClockDigits { .. })
        ));
    }

    #[test]
    fn malformed_idle_color_produces_correct_error_variant() {
        let raw = RawTheme {
            idle: "#0000000".into(),
            ..valid_raw_theme()
        };

        assert!(matches!(
            Theme::try_from(raw),
            Err(ParseThemeError::InvalidIdleValue { .. })
        ));
    }

    #[cfg(test)]
    fn valid_raw_stop() -> RawStop {
        RawStop {
            progress: 0.5,
            color: "#555555".into(),
        }
    }

    #[test]
    fn raw_stop_progress_value_too_high_produces_correct_error_variant() {
        let raw = RawStop {
            progress: 1.1,
            ..valid_raw_stop()
        };

        assert!(matches!(
            Stop::try_from(raw),
            Err(ParseStopError::InvalidProgressValue { .. })
        ));
    }

    #[test]
    fn raw_stop_progress_value_negative_produces_correct_error_variant() {
        let raw = RawStop {
            progress: -0.5,
            ..valid_raw_stop()
        };

        assert!(matches!(
            Stop::try_from(raw),
            Err(ParseStopError::InvalidProgressValue { .. })
        ));
    }
}
