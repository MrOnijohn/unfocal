use std::time::{Duration, Instant};

pub struct FocusTime {
    pub start: Instant,
    pub duration: Duration,
    pub quit: bool,
    pub paused: bool,
    pub paused_at: Option<Duration>,
}

impl FocusTime {
    pub fn toggle_paused(&mut self, elapsed: Duration) {
        if self.paused {
            // resume
            if let Some(paused_at) = self.paused_at {
                self.start = Instant::now() - paused_at;
            }
            self.paused = false;
            self.paused_at = None;
        } else {
            // pause
            self.paused = true;
            self.paused_at = Some(elapsed);
        }
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
        self.quit = false;
        self.paused = true;
        self.paused_at = Some(Duration::ZERO);
    }
}

pub struct TermGuard;
impl Drop for TermGuard {
    fn drop(&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen);
    }
}
