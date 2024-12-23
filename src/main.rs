mod app;
mod handler;
mod tui;

mod ui;
use app::{App, AppResult, LuaApp};
use crossterm::event::{self, Event};
use handler::handle_key_events;
use mlua::ObjectLike;
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

    // Get all simulation files from the simulations directory
    let simulation_files = std::fs::read_dir("src/simulations_lua")?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_dir() {
                Some(path.join("simulation.lua").to_str()?.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    app.lock().unwrap().possible_simulations = simulation_files;

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

    // Create a channel for sending events
    let (tx, rx) = std::sync::mpsc::channel();

    // Spawn a thread for handling input
    thread::spawn(move || {
        while app.lock().unwrap().running {
            if let Ok(event) = event::read() {
                tx.send(event).unwrap();
            }
        }
    });

    let mut lua_app = LuaApp::new();
    // Set up the initial simulation
    {
        let mut app_guard = app_clone.lock().unwrap();

        let noise_idx = app_guard
            .possible_simulations
            .iter()
            .position(|x| x.contains("noise"))
            .unwrap_or(0);
        lua_app.current_simulation_idx = noise_idx;
        app_guard.current_simulation_idx = noise_idx;
        lua_app.load_simulation(&mut app_guard)?;
    }

    while app_clone.lock().unwrap().running {
        let simulation = lua_app.simulation_instance.as_ref().unwrap();

        {
            // Check if the terminal size has changed
            let tui_guard = tui_clone.lock().unwrap();
            let terminal_size = tui_guard.terminal.size()?;
            let width = terminal_size.width as usize;
            let height = terminal_size.height as usize;

            let mut app_guard = app_clone.lock().unwrap();
            let current_height = app_guard.particles.len();
            let current_width = app_guard.particles[0].len();
            if width != current_width || height != current_height {
                app_guard.change_dimensions(width as usize, height as usize);
                simulation.call_method("set_particles", app_guard.particles.clone())?;
            }
        }

        // Get the new particles
        let particles: Vec<Vec<f64>> = simulation.call_method("simulate", ())?;
        {
            // Updates the particles
            let mut app_guard = app_clone.lock().unwrap();
            app_guard.particles = particles;
        }

        {
            // Draw the particles
            let mut app_guard = app_clone.lock().unwrap();
            tui_clone
                .lock()
                .unwrap()
                .draw(&mut app_guard)
                .expect("Failed to draw UI");
        }

        // Sleep for a short period to control the simulation speed
        thread::sleep(Duration::from_millis(250));

        // Handle events
        // It is done here to exit the loop if the user presses 'q' without needing to wait for the next frame
        while let Ok(event) = rx.try_recv() {
            let mut app_guard = app_clone.lock().unwrap();
            match event {
                Event::Key(event) => {
                    handle_key_events(event, &mut app_guard, &mut lua_app)?;
                }
                _ => {}
            }
        }
    }

    // Exit the user interface.
    tui_clone.lock().unwrap().exit()?;
    Ok(())
}
