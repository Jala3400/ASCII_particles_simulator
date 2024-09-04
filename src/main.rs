mod app;
mod handler;
mod simulations;
mod tui;

mod ui;
use app::{App, AppResult};
use crossterm::event::{self, Event};
use handler::{handle_key_events, handle_resize_event};
use ratatui::prelude::*;
use std::{
    io,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};
use tui::Tui;

fn main() -> AppResult<()> {
    // Create an application.
    let app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.init()?;

    // Create a RwLock wrapped in an Arc, which protects the shared data (an integer in this case).
    let shared_data = Arc::new(RwLock::new(app));
    // Clone the Arc to share it with the first thread
    let writer_data = Arc::clone(&shared_data);
    // Clone the Arc to share it with the reader thread
    let reader_data = Arc::clone(&shared_data);

    // Spawn a thread for handling input
    thread::spawn(move || {
        while writer_data.read().unwrap().running {
            if let Ok(event) = event::read() {
                match event {
                    Event::Key(event) => handle_key_events(event, writer_data.write().unwrap()).unwrap(),
                    Event::Resize(x, y) => handle_resize_event(writer_data.write().unwrap(), x, y).unwrap(),
                    _ => {}
                }
            }
        }
    });

    while reader_data.read().unwrap().running {
        // Draw the particles
        tui.draw(&mut reader_data.read().unwrap().clone())?;

        // Sleep for a short period to control the simulation speed
        thread::sleep(Duration::from_millis(100));
    }
    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
