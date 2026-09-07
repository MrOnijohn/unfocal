mod app;
mod clock;
mod color;
mod config;
mod message;
mod notifications;
mod settings;
mod timer;

pub use app::{Unfocol, OmarchyWatcher};
pub use color::{Color, Stop, Theme};
pub use config::{
    Config, DEFAULT_THEME, DEFAULT_THEMES, Settings, SettingsLoadingOutcome, load_settings,
    load_themes, sanitize_selected_theme, get_or_create_config_dir,ensure_themes_toml_exists, omarchy_colors_toml, omarchy_theme, omarchy_current
};
pub use message::{Message, Severity};
pub use settings::write_atomic;
pub use timer::Timer;
