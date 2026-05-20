use std::time::Instant;

use eframe::egui;
use unfocol::Unfocol;

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([100.0, 1080.0])
            .with_position([1820.0, 0.0])
            .with_decorations(false)
            .with_resizable(true)
            .with_movable_by_background(true),
        ..Default::default()
    };
    eframe::run_native(
        "Unfocol",
        options,
        Box::new(|cc| Ok(Box::<Unfocol<fn() -> Instant>>::default())),
    )
}
