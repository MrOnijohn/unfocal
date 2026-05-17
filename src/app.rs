use std::time::{Duration, Instant};

use eframe::egui;

use crate::Theme;
use crate::Timer;
use crate::color::Color;
use crate::timer::SessionState;

pub struct Unfocol<F: Fn() -> Instant> {
    timer: Timer<F>,
    theme: Theme,
}

impl Unfocol<fn() -> Instant> {
    fn handle_inputs(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Space) {
                self.timer.toggle();
            }
            if i.key_pressed(egui::Key::S) || i.key_pressed(egui::Key::Comma) {
                todo!(); // Open settings
            }
            if i.key_pressed(egui::Key::R) {
                self.timer.reset();
            }
            if i.key_pressed(egui::Key::Q) {
                todo!(); // Quit
            }
            if i.key_pressed(egui::Key::D) {
                todo!(); // Debug mode: display progress(t), remaining, and Color
            }
            if i.key_pressed(egui::Key::T) {
                todo!(); // Switch theme, how?
            }
            if i.key_pressed(egui::Key::A) {
                todo!(); // (A)ccelerate time/scrub forward 10 seconds (or more?)
            }
        });
    }
}

impl Default for Unfocol<fn() -> Instant> {
    fn default() -> Self {
        Self {
            timer: Timer::default(),
            theme: Theme::default(),
        }
    }
}

impl eframe::App for Unfocol<fn() -> Instant> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.handle_inputs(ui.ctx());

        self.timer.update_state();

        let current_color: Color = match self.timer.state {
            SessionState::Idle { .. } => self.theme.idle,
            SessionState::Running { .. } => {
                let t: f32 = self.timer.progress();
                self.theme.current_color(t)
            }
        };
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(current_color.into()))
            .show_inside(ui, |_ui| {});

        ui.ctx().request_repaint_after(Duration::from_millis(500));
    }
}
