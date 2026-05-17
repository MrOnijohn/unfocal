use std::time::{Duration, Instant};

use eframe::egui;

pub struct Unfocol {
    color: egui::Color32,
    last_change: Instant,
}

impl Default for Unfocol {
    fn default() -> Self {
        Self {
            color: egui::Color32::RED,
            last_change: Instant::now(),
        }
    }
}

impl eframe::App for Unfocol {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(self.color))
            .show_inside(ui, |_ui| {});

        ui.ctx().request_repaint_after(Duration::from_millis(500));
        if ui.ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            todo!();
        }
    }
}
