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

        if self.last_change.elapsed() > Duration::from_secs(1) {
            if self.color == egui::Color32::RED {
                self.color = egui::Color32::BLUE;
            } else {
                self.color = egui::Color32::RED;
            }
            self.last_change = Instant::now();
        }
        ui.ctx().request_repaint_after(Duration::from_millis(500));
    }
}
