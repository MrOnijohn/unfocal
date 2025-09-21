use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    prelude::*,
    widgets::{Block, Paragraph},
};
use std::{
    io,
    thread::sleep,
    time::{Duration, Instant},
};

struct FocusTime {
    start: Instant,
    duration: Duration,
    quit: bool,
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let mut app = FocusTime {
        start: Instant::now(),
        duration: Duration::new(5, 0),
        quit: false,
    };

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    loop {
        let elapsed = app.start.elapsed();
        let remaining = app.duration.saturating_sub(elapsed);

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)])
                .split(f.size());

            let color_field = Block::default().style(Style::default().bg(Color::Green));
            f.render_widget(color_field, chunks[0]);

            let clock = Paragraph::new(format!("{:?} s left", remaining.as_secs()))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Center);
            f.render_widget(clock, chunks[1]);
        })?;

        if remaining.is_zero() {
            app.quit = true;
        }
        if app.quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
