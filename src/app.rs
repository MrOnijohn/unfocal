use std::path::PathBuf;
use std::time::{Duration, Instant};

use eframe::egui;

use crate::Timer;
use crate::color::Color;
use crate::config::Config;
use crate::timer::SessionState;
use crate::{Message, Theme};

pub struct Unfocol<F: Fn() -> Instant> {
    pub timer: Timer<F>,
    pub settings_t: f32,
    pub settings_idle: bool,
    pub mouse_over: bool,
    pub config: Config,
    pub config_dir: PathBuf,
    pub messages: Vec<Message>,
}

impl Unfocol<fn() -> Instant> {
    pub fn new(config: Config, config_dir: PathBuf, messages: Vec<Message>) -> Self {
        Self {
            timer: Timer::default(),
            settings_t: 0.0,
            settings_idle: false,
            mouse_over: false,
            config,
            config_dir,
            messages,
        }
    }

    fn handle_inputs(&mut self, ctx: &egui::Context) {
        let (toggle_state, open_settings, reset_timer, quit, mouse_over) = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::Space),
                i.key_pressed(egui::Key::S) || i.key_pressed(egui::Key::Comma),
                i.key_pressed(egui::Key::R),
                i.key_pressed(egui::Key::Q),
                i.pointer.hover_pos().is_some(),
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
        self.mouse_over = mouse_over;
    }

    fn render_focus_window(&mut self, current_color: Color, ui: &mut egui::Ui) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(current_color.into()))
            .show_inside(ui, |_ui| {});
    }

    fn get_current_color(&self) -> Color {
        if self.config.settings.show_settings {
            if self.settings_idle {
                self.active_theme().idle
            } else {
                self.active_theme().current_color(self.settings_t)
            }
        } else {
            match self.timer.state {
                SessionState::Idle { .. } => self.active_theme().idle,
                SessionState::Running { .. } => {
                    let t: f32 = self.timer.progress();
                    self.active_theme().current_color(t)
                }
            }
        }
    }

    pub fn active_theme(&self) -> &Theme {
        &self.config.themes[&self.config.settings.selected_theme]
    }
}

impl eframe::App for Unfocol<fn() -> Instant> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.handle_inputs(ui.ctx());

        self.timer.update_state();

        self.render_settings(ui.ctx());
        self.render_clock(ui.ctx());

        let current_color: Color = self.get_current_color();
        self.render_focus_window(current_color, ui);

        let nanos = self.timer.remaining().subsec_nanos();
        let until_repaint = if nanos == 0 {
            Duration::from_secs(1)
        } else {
            Duration::from_nanos(nanos as u64)
        };
        if matches!(self.timer.state, SessionState::Running { .. }) {
            ui.ctx()
                .request_repaint_after(until_repaint + Duration::from_millis(1));
        }
    }
}
