mod app;
mod handler;
mod simulations;
mod tui;

mod ui;
use app::{App, AppResult, CurrentScreen};
use crossterm::event::{self, Event};
use handler::handle_key_events;
use ratatui::prelude::*;
use std::{io, sync::mpsc, thread, time::Duration};
use tui::Tui;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;

    // Get the size of the terminal screen
    let terminal_size = terminal.size()?;
    let screen_width = terminal_size.width as usize;
    let screen_height = terminal_size.height as usize;
    app.fire_data.change_dimensions(screen_width, screen_height);

    let mut tui = Tui::new(terminal);
    tui.init()?;

    let mut app_2 = app.clone();

    let (tx, rx) = mpsc::channel();

    // Spawn a thread for handling input
    thread::spawn(move || {
        while app.running {
            if let Ok(Event::Key(event)) = event::read() {
                if handle_key_events(event, &mut app) {
                    tx.send(app.clone()).unwrap();
                }
            }
        }
    });

    while app_2.running {
        // Draw the particles
        tui.draw(&mut app_2)?;

        // Receive the application state from the input thread
        if let Ok(new_app) = rx.try_recv() {
            app_2 = new_app;
        }

        // Sleep for a short period to control the simulation speed
        thread::sleep(Duration::from_millis(100));
    }
    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
