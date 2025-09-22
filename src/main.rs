use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use dirs;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    prelude::*,
    widgets::{Block, Paragraph},
};
use std::{
    io,
    sync::mpsc,
    time::{Duration, Instant},
};
use unfocol_utils::color::{extract_normal_colors, gradient_color, load_theme_or_default};

mod app;
use app::FocusTime;

fn main() -> anyhow::Result<()> {
    let mut last_reload = Instant::now();

    let mut app = FocusTime {
        start: Instant::now(),
        duration: Duration::from_secs(30 * 60), // Default 30 * 60 seconds, ie 30 minutes
        quit: false,
        paused: true,
        paused_at: Some(Duration::ZERO),
    };

    let path = dirs::home_dir()
        .unwrap()
        .join(".config/omarchy/current/theme/alacritty.toml"); // Get color values from the theme

    // Set default colors if theme colors aren't found
    let mut theme = load_theme_or_default(&path);
    let mut colors = extract_normal_colors(&theme);

    let stops = [
        (0.0, colors.green),
        (0.5, colors.yellow),
        (5.0 / 6.0, colors.red),
        (1.0, colors.black),
    ];

    // Set up watching for theme changes
    let parent = dirs::home_dir().unwrap().join(".config/omarchy/current");
    let (tx, rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, notify::Config::default())?;
    watcher.watch(&parent, RecursiveMode::NonRecursive)?;

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    loop {
        // Check for keyboard events
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

        // Check for updated theme
        if let Ok(Ok(_event)) = rx.try_recv() {
            let now = Instant::now();
            // Wait 200 ms after change to reload theme
            if now.duration_since(last_reload) > Duration::from_millis(200) {
                // reload theme file
                let new_theme = load_theme_or_default(&path);
                theme = new_theme;
                // re-extract RGBs
                colors = extract_normal_colors(&theme);
                last_reload = now;
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
                Color::Rgb(colors.cyan.0, colors.cyan.1, colors.cyan.2)
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

            let pad_top = Block::default().style(Style::default().bg(Color::Rgb(
                colors.black.0,
                colors.black.1,
                colors.black.2,
            )));
            f.render_widget(pad_top, inner_chunks[0]);

            let padded_text = format!("{:^7} ", formatted_time);

            let clock = Paragraph::new(padded_text)
                .style(
                    Style::default()
                        .fg(Color::Rgb(colors.white.0, colors.white.1, colors.white.2))
                        .bg(Color::Rgb(colors.black.0, colors.black.1, colors.black.2)),
                )
                .alignment(Alignment::Center);
            f.render_widget(clock, inner_chunks[1]);
            let pad_bottom = Block::default().style(Style::default().bg(Color::Rgb(
                colors.black.0,
                colors.black.1,
                colors.black.2,
            )));
            f.render_widget(pad_bottom, inner_chunks[2]);
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
