use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::Context;
use atomicwrites::replace_atomic;
use eframe::egui;
use eframe::egui::ViewportBuilder;
use eframe::egui::ViewportClass;
use eframe::egui::ViewportId;

use crate::Theme;
use crate::Timer;
use crate::color::Color;
use crate::config::Config;
use crate::timer::SessionState;

pub struct Unfocol<F: Fn() -> Instant> {
    timer: Timer<F>,
    theme: Theme,
    settings_t: f32,
    settings_idle: bool,
    config: Config,
    config_dir: PathBuf,
}

impl Unfocol<fn() -> Instant> {
    pub fn new(config: Config, config_dir: PathBuf) -> Self {
        Self {
            timer: Timer::default(),
            theme: Theme::default(),
            settings_t: 0.0,
            settings_idle: false,
            config,
            config_dir,
        }
    }

    fn handle_inputs(&mut self, ctx: &egui::Context) {
        let (toggle_state, open_settings, reset_timer, quit) = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::Space),
                i.key_pressed(egui::Key::S) || i.key_pressed(egui::Key::Comma),
                i.key_pressed(egui::Key::R),
                i.key_pressed(egui::Key::Q),
            )
        });
        if toggle_state {
            self.timer.toggle();
        }
        if open_settings {
            self.config.settings.show_settings = true;
        }
        if reset_timer {
            self.timer.reset();
        }
        if quit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn render_settings(&mut self, ctx: &egui::Context) {
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
                        self.save_settings().expect("Failed to save settings");
                    }

                    egui::Grid::new("settings_grid")
                        .min_col_width(200.0)
                        .spacing([16.0, 12.0])
                        .show(ui, |ui| {
                            let before = self.config.settings.selected_theme.clone();
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
                            if before != self.config.settings.selected_theme {
                                self.theme = self.config.themes
                                    [&self.config.settings.selected_theme]
                                    .clone();
                                self.save_settings().expect("Failed to save settings");
                            }
                            if ui
                                .add(
                                    egui::Slider::new(
                                        &mut self.config.settings.focus_time,
                                        5..=100,
                                    )
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
                        });
                });
            });
        }
    }

    fn save_settings(&self) -> Result<(), anyhow::Error> {
        let toml_str =
            toml::to_string(&self.config.settings).context("Parsing settings to toml")?;
        let tmp_file_path = self.config_dir.join(".settings.toml.tmp");
        let mut tmp_file = File::create(&tmp_file_path)
            .with_context(|| format!("Creating {}", tmp_file_path.display()))?;
        tmp_file
            .write_all(toml_str.as_bytes())
            .with_context(|| format!("Writing toml to {}", tmp_file_path.display()))?;

        let settings_toml = self.config_dir.join("settings.toml");
        replace_atomic(&tmp_file_path, &settings_toml)
            .context("Replacing settings.toml with .settings.toml.tmp")?;

        Ok(())
    }

    fn render_focus_window(&mut self, current_color: Color, ui: &mut egui::Ui) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(current_color.into()))
            .show_inside(ui, |_ui| {});
    }

    fn get_current_color(&self) -> Color {
        if self.config.settings.show_settings {
            if self.settings_idle {
                self.theme.idle
            } else {
                self.theme.current_color(self.settings_t)
            }
        } else {
            match self.timer.state {
                SessionState::Idle { .. } => self.theme.idle,
                SessionState::Running { .. } => {
                    let t: f32 = self.timer.progress();
                    self.theme.current_color(t)
                }
            }
        }
    }
}

impl eframe::App for Unfocol<fn() -> Instant> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.handle_inputs(ui.ctx());

        self.timer.update_state();

        self.render_settings(ui.ctx());

        let current_color: Color = self.get_current_color();
        self.render_focus_window(current_color, ui);

        if matches!(self.timer.state, SessionState::Running { .. }) {
            ui.ctx().request_repaint_after(Duration::from_secs(1));
        }
    }
}
