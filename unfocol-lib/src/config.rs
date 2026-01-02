use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub colors: ColorStopsConfig,
    pub transition: TransitionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorStopsConfig {
    pub start: String, // Starting color (hex)
    pub mid: String,
    pub end: String,
    pub paused: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionConfig {
    #[serde(default = "default_curve")]
    pub curve: CurveType,
    #[serde(default = "default_steps")]
    pub steps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurveType {
    Linear, // Linear slope
    Sigmoid, // Transitions slower at the beginning
}

impl Default for Config {
    fn default() -> Self {
        Self {
            colors: ColorStopsConfig {
                start: "#00ff00".into(),
                mid: "#ffff00".into(),
                end: "#ff0000".into(),
                paused: "#00ffff".into(),
            },
            transition: TransitionConfig::default(),
        }
    }
}

impl Default for TransitionConfig {
    fn default() -> Self {
        Self { curve: }
    }
}
