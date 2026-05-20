use std::time::{Duration, Instant};

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
    show_settings: bool,
    settings_t: f32,
    settings_idle: bool,
    config: Config,
}

impl Unfocol<fn() -> Instant> {
    fn handle_inputs(&mut self, ctx: &egui::Context) {
        let (toggle_state, open_settings, reset_timer, quit, debug_mode) = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::Space),
                i.key_pressed(egui::Key::S) || i.key_pressed(egui::Key::Comma),
                i.key_pressed(egui::Key::R),
                i.key_pressed(egui::Key::Q),
                i.key_pressed(egui::Key::D),
            )
        });
        if toggle_state {
            self.timer.toggle();
        }
        if open_settings {
            self.show_settings = true;
        }
        if reset_timer {
            self.timer.reset();
        }
        if quit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if debug_mode {
            todo!(); // Debug mode: display progress(t), remaining, and Color
        }
    }

    fn render_settings(&mut self, ctx: &egui::Context) {
        if self.show_settings {
            let viewport_id = ViewportId::from_hash_of("settings");
            let builder = ViewportBuilder::default()
                .with_title("Settings Viewport")
                .with_active(true)
                .with_decorations(true)
                .with_close_button(true)
                .with_inner_size([480.0, 200.0])
                .with_min_inner_size([480.0, 200.0]);
            let _class = ViewportClass::Immediate;

            ctx.show_viewport_immediate(viewport_id, builder, |ui, _class| {
                if let Some(cmd) = egui::ViewportCommand::center_on_screen(ctx) {
                    ctx.send_viewport_cmd(cmd);
                }
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.show_settings = false;
                        ui.close();
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
                            }
                            ui.add(
                                egui::Slider::new(&mut self.config.settings.focus_time, 10..=100)
                                    .text("Focus time duration"),
                            );
                            ui.end_row();
                            ui.add(
                                egui::Slider::new(&mut self.settings_t, 0.0..=1.0)
                                    .text("Preview focus color"),
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

    fn set_theme(&mut self, theme_name: String) {
        self.theme = self.config.themes[&theme_name].clone();
        self.config.settings.selected_theme = theme_name;
    }

    fn render_focus_window(&mut self, current_color: Color, ui: &mut egui::Ui) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(current_color.into()))
            .show_inside(ui, |_ui| {});
    }

    fn get_current_color(&self) -> Color {
        if self.show_settings {
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

impl Default for Unfocol<fn() -> Instant> {
    fn default() -> Self {
        Self {
            timer: Timer::default(),
            theme: Theme::default(),
            show_settings: true,
            settings_t: 0.0,
            settings_idle: false,
            config: Config::default(),
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
