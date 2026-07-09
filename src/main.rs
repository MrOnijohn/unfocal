use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Context;
use atomicwrites::replace_atomic;
use directories::ProjectDirs;
use eframe::{Renderer, egui};
use log::info;
use unfocol::{Config, DEFAULT_THEMES, Unfocol, load_settings, load_themes};

fn main() -> eframe::Result {
    env_logger::init();

    info!("Setting up configuration paths");
    let config_dir = get_or_create_config_dir();
    let themes_toml = config_dir.join("themes.toml");
    ensure_themes_toml_exists(&themes_toml, &config_dir).expect("Could not create themes.toml");
    let settings_toml = config_dir.join("settings.toml");

    info!("Loading themes from {}", themes_toml.display());
    // TODO: Do something with the errors here
    let (themes, _) = load_themes(themes_toml);
    info!("Loading settings from {}", settings_toml.display());
    let settings = load_settings(settings_toml);

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
        Box::new(|_cc| Ok(Box::new(Unfocol::new(config, config_dir)))),
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
        let tmp_file_path = config_dir.join(".themes.toml.tmp");
        let mut tmp_file = File::create(&tmp_file_path)
            .with_context(|| format!("Creating {}", tmp_file_path.display()))?;
        tmp_file
            .write_all(DEFAULT_THEMES.as_bytes())
            .with_context(|| format!("Writing default themes to {}", tmp_file_path.display()))?;
        replace_atomic(&tmp_file_path, themes_toml)
            .context("Replacing themes.toml with .themes.toml.tmp")?;

        Ok(())
    } else {
        info!("Found themes.toml");
        Ok(())
    }
}
