use crate::config::{
    LoadSettingsError, LoadThemesError, ParseStopError, ParseThemeError, SettingsLoadingOutcome,
};

fn default_themes_loaded() -> &'static str {
    "Default themes have been loaded instead."
}

fn default_settings_loaded() -> &'static str {
    "Default settings have been used instead."
}

fn correct_progress_values_and_restart() -> &'static str {
    "Correct progress values and restart to try again."
}

fn correct_hex_code_and_restart() -> &'static str {
    "Correct hex code and restart to try again."
}

enum Severity {
    Error,
    Warning,
    Info,
}
pub struct Message {
    severity: Severity,
    message: String,
}

impl Message {
    fn from_load_themes_error(error: LoadThemesError) -> Self {
        match error {
            LoadThemesError::FileUnreadable { file, io_error } => {
                let message = format!(
                    "Loading themes from {} failed with error {}.\n{}",
                    file.display(),
                    io_error,
                    default_themes_loaded(),
                );
                let severity = Severity::Error;
                Self { severity, message }
            }
            LoadThemesError::ParseTomlFailed(parsing_error) => {
                let message = format!(
                    "Parsing themes.toml failed with error \n{}\n{}",
                    parsing_error,
                    default_themes_loaded(),
                );
                let severity = Severity::Error;
                Self { severity, message }
            }
            LoadThemesError::NoThemesDefined => {
                let message = format!(
                    "Found no themes in themes.toml. {}",
                    default_themes_loaded()
                );
                let severity = Severity::Error;
                Self { severity, message }
            }
            LoadThemesError::InvalidTheme {
                name,
                parse_theme_error,
            } => {
                let (details, advice) = match &parse_theme_error {
                    ParseThemeError::InvalidIdleValue { .. }
                    | ParseThemeError::InvalidClockBG { .. }
                    | ParseThemeError::InvalidClockDigits { .. } => (
                        parse_theme_error.to_string(),
                        correct_hex_code_and_restart().to_string(),
                    ),
                    ParseThemeError::InvalidStop(parse_stop_error) => {
                        let details = format!("Invalid stop: {parse_stop_error}");
                        let advice = match parse_stop_error {
                            ParseStopError::InvalidProgressValue { .. } => {
                                correct_progress_values_and_restart().to_string()
                            }
                            ParseStopError::InvalidHexValue { .. } => {
                                correct_hex_code_and_restart().to_string()
                            }
                        };
                        (details, advice)
                    }
                    ParseThemeError::InvalidProgressBounds { .. } => (
                        parse_theme_error.to_string(),
                        correct_progress_values_and_restart().to_string(),
                    ),
                    ParseThemeError::NonMonotonicStops { .. } => (
                        parse_theme_error.to_string(),
                        correct_progress_values_and_restart().to_string(),
                    ),
                    ParseThemeError::TooFewStops { .. } => (
                        parse_theme_error.to_string(),
                        "Make sure that the theme has at least two stops.".to_string(),
                    ),
                };
                let message = format!(
                    "The theme \"{}\" in themes.toml did not parse correctly with error: {}\n{}",
                    name, details, advice
                );
                let severity = Severity::Warning;
                Self { severity, message }
            }
        }
    }

    fn from_settings_loading_outcome(outcome: SettingsLoadingOutcome) -> Self {
        match outcome {
            SettingsLoadingOutcome::ParsedAndLoaded { corrections } => {
                let details: String = corrections
                    .into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");

                let message = format!(
                    "Loading settings from settings.toml needed some correction(s):\n{details}"
                );
                let severity = Severity::Warning;
                Self { message, severity }
            }
            SettingsLoadingOutcome::Defaulted {
                load_settings_error,
            } => {
                let message = match load_settings_error {
                    LoadSettingsError::FileUnreadable(error) => {
                        format!(
                            "Reading settings.toml returned the error\n{}\n{}",
                            error,
                            default_settings_loaded()
                        )
                    }
                    LoadSettingsError::ParseTomlFailed(error) => {
                        format!(
                            "Parsing the contents of settings.toml failed with the error\n{}\n{}",
                            error,
                            default_settings_loaded()
                        )
                    }
                };
                let severity = Severity::Error;
                Self { message, severity }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    #[test]
    fn missing_settings_toml_returns_correct_error() {
        let io_error = Error::new(ErrorKind::NotFound, "No such file or directory");
        let load_settings_error = LoadSettingsError::FileUnreadable(io_error);
        let outcome = SettingsLoadingOutcome::Defaulted {
            load_settings_error,
        };

        let message = Message::from_settings_loading_outcome(outcome);

        assert!(matches!(message.severity, Severity::Error));
        assert!(message.message.contains("No such file or directory"));
    }
}
