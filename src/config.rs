use crate::color::Theme;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RawTheme {
    color: String,
    progress: f32,
}

#[derive(Serialize, Deserialize)]
struct RawSetting {
    focus_time: u32,
    theme: String,
}
