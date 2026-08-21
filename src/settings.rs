use crate::Settings;
use crate::config::ShowClock;
use anyhow::Context;
use atomicwrites::replace_atomic;
use eframe::egui;
use eframe::egui::ViewportBuilder;
use eframe::egui::ViewportClass;
use eframe::egui::ViewportId;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

use crate::app::Unfocol;

impl Unfocol<fn() -> Instant> {
    fn save_settings(&self) -> Result<(), anyhow::Error> {
        save_settings(&self.config.settings, &self.config_dir)
    }

    pub fn render_settings(&mut self, ctx: &egui::Context) {
        if self.config.settings.show_settings {
            let viewport_id = ViewportId::from_hash_of("settings");
            let builder = ViewportBuilder::default()
                .with_title("Unfocol settings")
                .with_active(true)
                .with_decorations(true)
                .with_close_button(true)
                .with_inner_size([480.0, 200.0])
                .with_min_inner_size([50.0, 50.0]);
            let _class = ViewportClass::Immediate;

            ctx.show_viewport_immediate(viewport_id, builder, |ui, _class| {
                if let Some(cmd) = egui::ViewportCommand::center_on_screen(ctx) {
                    ctx.send_viewport_cmd(cmd);
                }
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    let should_close = ctx.input(|i| {
                        i.key_pressed(egui::Key::Escape) || i.viewport().close_requested()
                    });
                    if should_close {
                        self.config.settings.show_settings = false;
                        self.settings_t = 0.0;
                        self.save_settings().expect("Failed to save settings");
                    }

                    egui::Grid::new("settings_grid")
                        .min_col_width(200.0)
                        .spacing([16.0, 12.0])
                        .show(ui, |ui| {
                            let theme_before = self.config.settings.selected_theme.clone();
                            egui::ComboBox::from_label("Choose theme")
                                .selected_text(format!("{:?}", self.config.settings.selected_theme))
                                .show_ui(ui, |ui| {
                                    for theme in self.config.themes.keys() {
                                        ui.selectable_value(
                                            &mut self.config.settings.selected_theme,
                                            theme.clone(),
                                            theme,
                                        );
                                    }
                                });
                            if theme_before != self.config.settings.selected_theme {
                                self.save_settings().expect("Failed to save settings");
                            }
                            ui.end_row();
                            if ui
                                .add(
                                    egui::Slider::new(&mut self.config.settings.focus_time, 1..=99)
                                        .text("Focus time duration"),
                                )
                                .changed()
                            {
                                self.save_settings().expect("Failed to save settings");
                            }
                            ui.end_row();
                            ui.add(
                                egui::Slider::new(&mut self.settings_t, 0.0..=1.0)
                                    .text("Preview focus colors"),
                            );
                            ui.add(egui::Checkbox::new(
                                &mut self.settings_idle,
                                "Preview idle color",
                            ));
                            ui.end_row();
                            let clock_before = self.config.settings.show_clock.clone();
                            egui::ComboBox::from_label("Show remaining time")
                                .selected_text(format!("{:?}", self.config.settings.show_clock))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.config.settings.show_clock,
                                        ShowClock::OnMouseOver,
                                        "On mouse over",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.settings.show_clock,
                                        ShowClock::Never,
                                        "Never",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.settings.show_clock,
                                        ShowClock::Always,
                                        "Always",
                                    );
                                });
                            if clock_before != self.config.settings.show_clock {
                                self.save_settings().expect("Failed to save settings")
                            }
                        });
                });
            });
        }
    }
}

pub fn write_atomic(toml_str: &str, final_file_path: &Path) -> Result<(), anyhow::Error> {
    let file_name = final_file_path
        .file_name()
        .and_then(|name| name.to_str())
        .with_context(|| format!("Getting file name from {}", final_file_path.display()))?;
    let tmp_file_path = final_file_path.with_file_name(format!(".{file_name}.tmp"));
    let mut tmp_file = File::create(&tmp_file_path)
        .with_context(|| format!("Creating {}", tmp_file_path.display()))?;
    tmp_file
        .write_all(toml_str.as_bytes())
        .with_context(|| format!("Writing toml to {}", tmp_file_path.display()))?;
    replace_atomic(&tmp_file_path, final_file_path)
        .with_context(|| format!("Replacing {}", final_file_path.display()))
}

fn save_settings(settings: &Settings, config_dir: &Path) -> Result<(), anyhow::Error> {
    let toml_str = toml::to_string(settings).context("Parsing settings to toml")?;
    let final_file_path = config_dir.join("settings.toml");
    write_atomic(&toml_str, &final_file_path)?;
    Ok(())
}
