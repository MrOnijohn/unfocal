use crate::config::{
    LoadSettingsError, LoadThemesError, ParseStopError, ParseThemeError, SettingsLoadingOutcome,
};

fn default_themes_loaded() -> &'static str {
    "Default themes have been loaded instead."
}

fn correct_progress_values_and_restart() -> &'static str {
    "Correct progress values and restart to try again."
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
                    "Loading themes from {} failed with error {}. {}",
                    file.display(),
                    io_error,
                    default_themes_loaded(),
                );
                let severity = Severity::Error;
                Self { severity, message }
            }
            LoadThemesError::ParseTomlFailed {
                toml_contents: _,
                parsing_error,
            } => {
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
                let advice = match parse_theme_error {
                    ParseThemeError::InvalidIdleValue { .. }
                    | ParseThemeError::InvalidClockBG { .. }
                    | ParseThemeError::InvalidClockDigits { .. } => {
                        "Change to a valid hex code and restart to try again.".to_string()
                    }
                    ParseThemeError::InvalidStop { .. } => {
                        correct_progress_values_and_restart().to_string()
                    }
                    ParseThemeError::InvalidProgressBounds { .. } => {
                        correct_progress_values_and_restart().to_string()
                    }
                    ParseThemeError::NonMonotonicStops { .. } => {
                        correct_progress_values_and_restart().to_string()
                    }
                    ParseThemeError::TooFewStops { .. } => {
                        "Make sure that the theme has at least two stops.".to_string()
                    }
                };
                let message = format!(
                    "The theme {} in themes.toml did not parse correctly with error {}.\n{}",
                    name, parse_theme_error, advice
                );
                let severity = Severity::Error;
                Self { severity, message }
            }
        }
    }

    fn from_settings_loading_outcome(outcome: SettingsLoadingOutcome) -> Self {
        match outcome {
            SettingsLoadingOutcome::ParsedAndLoaded { corrections } => {
                todo!();
            }
            SettingsLoadingOutcome::Defaulted { error } => {
                todo!();
            }
        }
    }
}

#[cfg(test)]
mod tests {}
