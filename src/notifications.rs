use eframe::egui;
use eframe::egui::ViewportBuilder;
use eframe::egui::ViewportClass;
use eframe::egui::ViewportId;
use std::time::Instant;

use crate::Unfocol;

impl Unfocol<fn() -> Instant> {
    pub fn display_notifications(&mut self, ctx: &egui::Context) {
        if !self.messages.is_empty() {
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
                        if should_close {}
                    });
                });
            };
        }
    }
}
