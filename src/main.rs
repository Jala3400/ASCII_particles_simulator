use std::{error::Error, io};

use ratatui::{
    crossterm::{
        cursor::SetCursorStyle,
        event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::*,
};

use color_eyre::config::HookBuilder;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

mod app;
use app::App;
mod ui;
use ui::ui;

fn main() -> Result<()> {
    init_error_hooks()?;
    let mut terminal = init_terminal()?;

    let mut app = App::new();
    let res = run_tui(&mut terminal, &mut app);

    restore_terminal()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn init_error_hooks() -> color_eyre::Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info);
    }));
    Ok(())
}

/// Initializes the terminal.
fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture,
        SetCursorStyle::BlinkingBar
    )?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(io::stdout());

    let mut terminal = Terminal::new(backend)?;
    terminal.show_cursor()?;

    Ok(terminal)
}

/// Resets the terminal.
fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}

pub fn run_tui<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if event::poll(std::time::Duration::from_millis(200))? {
            // 16ms = 60 fps
            if let event::Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    // Skip events that are not KeyEventKind::Press
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') => return Ok(false),
                    _ => {}
                }
            }
        }
    }
}
