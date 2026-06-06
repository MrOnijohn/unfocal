use crate::app::Unfocol;
use eframe::egui::{self, Align2, Area, Frame, Id, Vec2};
use std::time::{Duration, Instant};

impl Unfocol<fn() -> Instant> {
    pub fn render_clock(&self, ctx: &egui::Context) {
        match self.show_clock {
            false => {}
            true => {
                Area::new(Id::new("clock_overlay"))
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                    .interactable(false)
                    .show(ctx, |ui| {
                        Frame::new()
                            .fill(self.theme.clock_bg.into())
                            .show(ui, |ui| ui.label("Hello Clock!"))
                    });
            }
        }
    }
}
