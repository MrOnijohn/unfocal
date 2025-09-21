use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

struct FocusTime {
    start: Instant,
    duration: Duration,
    quit: bool,
}

fn main() -> io::Result<()> {
    execute!(io::stdout(), EnterAlternateScreen)?;

    loop {
        let app = FocusTime {
            start: Instant::now(),
            duration: Duration::new(30, 0),
            quit: false,
        };
        let elapsed = app.start.elapsed();
        println!("Elapsed: {:?}", elapsed);

        sleep(Duration::from_millis(100));
        break; // For now
    }

    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
