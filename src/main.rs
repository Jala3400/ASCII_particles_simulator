use std::{error::Error, io};

use color_eyre::config::HookBuilder;
use crossterm::event::KeyEvent;
use ratatui::{
    crossterm::{
        cursor::SetCursorStyle,
        event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::*,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

mod app;
use app::{App, CurrentScreen};
mod ui;
use ui::ui;
mod simulations;

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
                use KeyCode::*;
                match key.code {
                    Char('q') => return Ok(false),
                    Char('n') | Char('1') => app.current_screen = CurrentScreen::Noise,
                    Char('f') | Char('2') => app.current_screen = CurrentScreen::Fire,
                    Tab => {
                        app.particles_index =
                            (app.particles_index + 1) % app.particles_styles.len();
                    }

                    _ => match app.current_screen {
                        CurrentScreen::Noise => handle_noise_key(key, app),
                        CurrentScreen::Fire => handle_fire_key(key, app),
                    },
                }
            }
            if let event::Event::Resize(width, height) = event::read()? {
                app.fire_data.particles = vec![vec![0.0; width as usize]; height as usize];
            }
        }
    }
}

fn handle_noise_key(key: KeyEvent, app: &mut App) {
    use KeyCode::*;
    match key.code {
        Char('+') => {
            app.noise_data.noise_intensity += 0.1;
            app.show_info = true;
        }
        Char('-') => {
            app.noise_data.noise_intensity -= 0.1;
            app.show_info = true;
        }
        Up => {
            app.noise_data.min_brightness += 0.1;
            app.noise_data.max_brightness += 0.1;
            app.show_info = true;
        }
        Down => {
            app.noise_data.min_brightness -= 0.1;
            app.noise_data.max_brightness -= 0.1;
            app.show_info = true;
        }
        Right => {
            app.noise_data.min_brightness -= 0.1;
            app.noise_data.max_brightness += 0.1;
            app.show_info = true;
        }
        Left => {
            app.noise_data.min_brightness += 0.1;
            app.noise_data.max_brightness -= 0.1;
            if app.noise_data.min_brightness > app.noise_data.max_brightness {
                let tmp = app.noise_data.min_brightness;
                app.noise_data.min_brightness = app.noise_data.max_brightness;
                app.noise_data.max_brightness = tmp;
            }
            app.show_info = true;
        }
        Char('r') => {
            let new_app = App::new();
            *app = new_app;
        }
        _ => {
            app.show_info = false;
        }
    }
}

fn handle_fire_key(key: KeyEvent, app: &mut App) {
    use KeyCode::*;
    match key.code {
        Char('q') => return,
        _ => {}
    }
}
