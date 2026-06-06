use crate::app::Unfocol;
use eframe::egui::{self, Align2, Area, Frame, Id, Vec2};
use std::time::{Duration, Instant};

impl Unfocol<fn() -> Instant> {
    pub fn render_clock(&self, ctx: &egui::Context) {
        let time = self.timer.remaining();
        let clock = parse_duration(time);

        match self.show_clock {
            false => {}
            true => {
                Area::new(Id::new("clock_overlay"))
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                    .interactable(false)
                    .show(ctx, |ui| {
                        Frame::new()
                            .fill(self.theme.clock_bg.into())
                            .show(ui, |ui| ui.label(clock))
                    });
            }
        }
    }
}

fn parse_duration(time: Duration) -> String {
    let total_secs = time.as_secs_f32().ceil();
    todo!();
}

#[cfg(test)]
mod tests {
    use csscolorparser::parse;

    use crate::clock;

    use super::*;

    const ONE_SEC: Duration = Duration::from_secs(1);

    #[test]
    fn sevenfivefour_seconds_returns_12_34() {
        let time = Duration::from_secs(754);
        let clock = parse_duration(time);

        assert_eq!(clock, "12:34".to_string());
    }

    #[test]
    fn five_seconds_returns_00_05() {
        let time = Duration::from_secs(5);
        let clock = parse_duration(time);

        assert_eq!(clock, "00:05".to_string())
    }

    #[test]
    fn sixtyfive_seconds_returns_01_05() {
        let time = Duration::from_secs(65);
        let clock = parse_duration(time);

        assert_eq!(clock, "01:05".to_string());
    }

    #[test]
    fn fiftynine_seconds_returns_00_59() {
        let time = Duration::from_secs(59);
        let clock = parse_duration(time);

        assert_eq!(clock, "00:59".to_string());
    }

    #[test]
    fn sixty_seconds_returns_01_00() {
        let time = Duration::from_secs(60);
        let clock = parse_duration(time);

        assert_eq!(clock, "01:00".to_string());
    }

    #[test]
    fn zero_returns_00_00() {
        let time = Duration::from_secs(0);
        let clock = parse_duration(time);

        assert_eq!(clock, "00:00".to_string());
    }

    #[test]
    fn threfivefourzero_returns_59_00() {
        let time = Duration::from_secs(3599);
        let clock = parse_duration(time);

        assert_eq!(clock,)
    }
}
