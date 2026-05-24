use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::time::Instant;

use directories::ProjectDirs;
use eframe::egui;
use log::{error, info, warn};
use unfocol::{Theme, Unfocol, load_themes};

fn main() -> eframe::Result {
    env_logger::init();
    let config_dir = get_or_create_config_dir();
    let themes_toml = config_dir.join("themes.toml");

    let themes: HashMap<String, Theme> = load_themes(themes_toml).unwrap_or_else(|e| {
        warn!("Themes loading failed: {e} Using default.");
        let mut default_themes: HashMap<String, Theme> = HashMap::new();
        default_themes.insert("default".to_string(), Theme::default());
        default_themes
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([100.0, 1080.0])
            .with_position([1820.0, 0.0])
            .with_decorations(false)
            .with_resizable(true)
            .with_movable_by_background(true),
        persist_window: true,
        ..Default::default()
    };
    eframe::run_native(
        "Unfocol",
        options,
        Box::new(|cc| Ok(Box::<Unfocol<fn() -> Instant>>::default())),
    )
}

fn get_or_create_config_dir() -> PathBuf {
    if let Some(proj_dir) = ProjectDirs::from("com", "unfocol", "Unfocol") {
        create_dir_all(proj_dir.config_dir()).expect("Could not create config directory");
        proj_dir.config_dir().to_path_buf()
    } else {
        panic!("No home directory")
    }
}
