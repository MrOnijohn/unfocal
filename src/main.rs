use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use eframe::{Renderer, egui};
use log::{info, warn};
use unfocol::{
    Config, DEFAULT_THEME, DEFAULT_THEMES, Message, SettingsLoadingOutcome, Unfocol, load_settings,
    load_themes, sanitize_selected_theme, write_atomic,
};

fn main() -> eframe::Result {
    env_logger::init();

    info!("Setting up configuration paths");
    let config_dir = get_or_create_config_dir();
    let themes_toml = config_dir.join("themes.toml");
    ensure_themes_toml_exists(&themes_toml, &config_dir).expect("Could not create themes.toml");
    let settings_toml = config_dir.join("settings.toml");

    info!("Loading themes from {}", themes_toml.display());
    let (themes, load_theme_errors) = load_themes(themes_toml);
    info!("Loading settings from {}", settings_toml.display());
    let (mut settings, settings_loading_outcome) = load_settings(&settings_toml);

    if matches!(settings_loading_outcome, SettingsLoadingOutcome::FirstRun) {
        settings.show_welcome_message = true;
        info!("Writing settings.toml");
        let toml_str =
            toml::to_string(&settings).expect("Default settings should always serialize to TOML");
        if let Err(e) = write_atomic(&toml_str, &settings_toml) {
            warn!("Failed to write settings.toml to disk: {e:?}");
        }
    }

    let mut messages: Vec<Message> = load_theme_errors
        .into_iter()
        .map(Message::from_load_themes_error)
        .collect();
    messages.extend(Message::from_settings_loading_outcome(
        settings_loading_outcome,
    ));

    if settings.show_welcome_message {
        messages.push(Message::welcome_message());
    }

    debug_assert!(themes.contains_key(DEFAULT_THEME));
    let (settings, correction) = sanitize_selected_theme(settings, &themes);
    if let Some(correction) = correction {
        let message = Message::from_settings_correction(correction);
        messages.push(message);
    }

    let config = Config::new(themes, settings);

    info!("Starting app");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([100.0, 1080.0])
            .with_min_inner_size([10.0, 10.0])
            .with_position([1820.0, 0.0])
            .with_decorations(false)
            .with_resizable(true)
            .with_movable_by_background(true),
        persist_window: true,
        renderer: if cfg!(target_os = "linux") {
            info!("Linux detected, choosing Glow as renderer");
            Renderer::Glow
        } else {
            info!("Choosing Wgpu as renderer");
            Renderer::Wgpu
        },
        ..Default::default()
    };
    eframe::run_native(
        "Unfocol",
        options,
        Box::new(|_cc| Ok(Box::new(Unfocol::new(config, config_dir, messages)))),
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

fn ensure_themes_toml_exists(themes_toml: &Path, config_dir: &Path) -> Result<(), anyhow::Error> {
    if !themes_toml.exists() {
        info!("No themes.toml file found, creating default.");
        let toml_str = DEFAULT_THEMES;
        let final_file_path = config_dir.join("themes.toml");
        write_atomic(toml_str, &final_file_path)?;
        Ok(())
    } else {
        info!("Found themes.toml");
        Ok(())
    }
}
