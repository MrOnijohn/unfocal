use std::time::{Duration, Instant};

use eframe::egui;
use eframe::egui::ViewportBuilder;
use eframe::egui::ViewportClass;
use eframe::egui::ViewportId;

use crate::Theme;
use crate::Timer;
use crate::color::Color;
use crate::timer::SessionState;

pub struct Unfocol<F: Fn() -> Instant> {
    timer: Timer<F>,
    theme: Theme,
    show_settings: bool,
    settings_t: f32,
    selected_theme: String,
}

impl Unfocol<fn() -> Instant> {
    fn handle_inputs(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Space) {
                self.timer.toggle();
            }
            if i.key_pressed(egui::Key::S) || i.key_pressed(egui::Key::Comma) {
                self.show_settings = true;
            }
            if i.key_pressed(egui::Key::R) {
                self.timer.reset();
            }
            if i.key_pressed(egui::Key::Q) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            if i.key_pressed(egui::Key::D) {
                todo!(); // Debug mode: display progress(t), remaining, and Color
            }
        });
    }

    fn render_settings(&mut self, ctx: &egui::Context) {
        if self.show_settings {
            let viewport_id = ViewportId::from_hash_of("settings");
            let builder = ViewportBuilder::default()
                .with_title("Settings Viewport")
                .with_active(true)
                .with_decorations(true)
                .with_close_button(true);
            let class = ViewportClass::Immediate;
            ctx.show_viewport_immediate(viewport_id, builder, |ui, class| {
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.show_settings = false;
                        ui.close();
                    }
                    ui.label("Unfocol Settings");

                    ui.add(egui::Slider::new(&mut self.settings_t, 0.0..=1.0));
                    let theme_names = vec!["default", "nord", "gruvbox"];
                    egui::ComboBox::from_label("Choose theme")
                        .selected_text(format!("{:?}", self.selected_theme))
                        .show_ui(ui, |ui| {
                            for theme in theme_names {
                                ui.selectable_value(
                                    &mut self.selected_theme,
                                    theme.to_owned(),
                                    theme,
                                );
                            }
                        });
                });
            });
        }
    }

    fn render_focus_window(&mut self, current_color: Color, ui: &mut egui::Ui) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(current_color.into()))
            .show_inside(ui, |_ui| {});
    }
}

impl Default for Unfocol<fn() -> Instant> {
    fn default() -> Self {
        Self {
            timer: Timer::default(),
            theme: Theme::default(),
            show_settings: true,
            settings_t: 0.0,
            selected_theme: "default".to_string(),
        }
    }
}

impl eframe::App for Unfocol<fn() -> Instant> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.handle_inputs(ui.ctx());

        self.timer.update_state();

        self.render_settings(ui.ctx());

        let current_color: Color = match self.timer.state {
            SessionState::Idle { .. } => self.theme.idle,
            SessionState::Running { .. } => {
                let t: f32 = self.timer.progress();
                self.theme.current_color(t)
            }
        };

        self.render_focus_window(current_color, ui);

        if matches!(self.timer.state, SessionState::Running { .. }) {
            ui.ctx().request_repaint_after(Duration::from_secs(1));
        }
    }
}
