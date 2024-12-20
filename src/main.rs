mod app;
mod handler;
mod tui;

mod ui;
use app::{App, AppResult, LuaApp};
use crossterm::event::{self, Event};
use handler::handle_key_events;
use ratatui::prelude::*;
use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tui::Tui;

fn main() -> AppResult<()> {
    // Create an application.
    let app = Arc::new(Mutex::new(App::new()));

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;

    // Get the size of the terminal screen
    let terminal_size = terminal.size()?;
    let screen_width = terminal_size.width as usize;
    let screen_height = terminal_size.height as usize;
    app.lock()
        .unwrap()
        .change_dimensions(screen_width, screen_height);

    let tui = Arc::new(Mutex::new(Tui::new(terminal)));
    tui.lock().unwrap().init()?;

    let app_clone = Arc::clone(&app);
    let tui_clone = Arc::clone(&tui);

    // Spawn a thread for handling input
    thread::spawn(move || {
        while app.lock().unwrap().running {
            if let Ok(event) = event::read() {
                match event {
                    Event::Key(event) => {
                        let mut app_lock = app.lock().unwrap();
                        handle_key_events(event, &mut app_lock);
                        tui.lock().unwrap().draw(&mut app_lock).unwrap();
                    }
                    Event::Resize(_, _) => {
                        tui.lock().unwrap().draw(&mut app.lock().unwrap()).unwrap();
                    }
                    _ => {}
                }
            }
        }
    });

    let mut lua_app = LuaApp::new();
    lua_app.load_simulation("src/simulations_lua/noise/simulation.lua")?;

    while app_clone.lock().unwrap().running {
        // Example call later:
        let func: mlua::Function = lua_app.current_simulation.globals().get("Simulate")?;

        let particles: Vec<Vec<f64>> =
            func.call((app_clone.lock().unwrap().particles.clone(), "nil"))?;
        app_clone.lock().unwrap().particles = particles;
        // Draw the particles
        tui_clone
            .lock()
            .unwrap()
            .draw(&mut app_clone.lock().unwrap())?;

        // Sleep for a short period to control the simulation speed
        thread::sleep(Duration::from_millis(250));
    }
    // Exit the user interface.
    tui_clone.lock().unwrap().exit()?;
    Ok(())
}
