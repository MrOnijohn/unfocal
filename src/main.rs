use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use dirs;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    prelude::*,
    widgets::{Block, Paragraph},
};
use std::{
    io,
    time::{Duration, Instant},
};

use unfocol_utils::color::{gradient_color, hex_to_rgb, load_theme_or_default};

mod app;
use app::FocusTime;

fn main() -> io::Result<()> {
    let mut app = FocusTime {
        start: Instant::now(),
        duration: Duration::from_secs(60),
        quit: false,
        paused: true,
        paused_at: Some(Duration::ZERO),
    };
    let path = dirs::home_dir()
        .unwrap()
        .join(".config/omarchy/current/theme/alacritty.toml"); // Get color values from the theme

    // Set default colors if theme colors aren't found
    let theme = load_theme_or_default(&path);
    let cyan = hex_to_rgb(&theme.colors.normal.cyan);
    let green = hex_to_rgb(&theme.colors.normal.green);
    let yellow = hex_to_rgb(&theme.colors.normal.yellow);
    let red = hex_to_rgb(&theme.colors.normal.red);
    let black = hex_to_rgb(&theme.colors.normal.black);

    let stops = [(0.0, green), (0.5, yellow), (5.0 / 6.0, red), (1.0, black)];

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.quit = true,
                    KeyCode::Char(' ') => {
                        let elapsed_now = if app.paused {
                            app.paused_at.unwrap_or(Duration::ZERO)
                        } else {
                            app.start.elapsed()
                        };
                        app.toggle_paused(elapsed_now);
                    }
                    KeyCode::Char('r') => {
                        app.reset();
                        continue; // skip this iteration so we don't use stale elapsed
                    }
                    _ => {}
                }
            }
        }
        let elapsed = if app.paused {
            app.paused_at.unwrap_or(Duration::ZERO)
        } else {
            app.start.elapsed()
        };
        let remaining = app.duration.saturating_sub(elapsed);
        let total_secs = remaining.as_secs();
        let minutes = total_secs / 60;
        let seconds = total_secs % 60;
        let formatted_time = format!("{:02}:{:02}", minutes, seconds);

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)])
                .split(f.area());
            let color = if app.paused {
                Color::Rgb(cyan.0, cyan.1, cyan.2)
            } else {
                let ratio = (elapsed.as_secs_f32() / app.duration.as_secs_f32()).min(1.0);
                let rgb = gradient_color(ratio, &stops);
                Color::Rgb(rgb.0, rgb.1, rgb.2)
            };

            let color_field = Block::default().style(Style::default().bg(color));
            f.render_widget(color_field, chunks[0]);

            let clock_area = chunks[1];

            let inner_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // top padding
                    Constraint::Length(1), // timer display
                    Constraint::Length(1), // bottom padding
                ])
                .split(clock_area);
            let clock = Paragraph::new(formatted_time)
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Center);
            f.render_widget(clock, inner_chunks[1]);
        })?;

        if remaining.is_zero() {
            app.reset();
            continue; // skip stale frame
        }

        if app.quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
