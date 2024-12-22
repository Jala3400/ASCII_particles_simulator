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
    lua_app.load_simulation(
        // app_clone
        //     .lock()
        //     .unwrap()
        //     .possible_simulations
        //     .get(app_clone.lock().unwrap().current_simulation_idx)
        //     .unwrap(),
        "src/simulations_lua/fire/simulation.lua",
        app_clone.lock().unwrap().particles.clone(),
    )?;

    while app_clone.lock().unwrap().running {
        let (current_simulation_idx, particles) = {
            let app_guard = app_clone.lock().unwrap();
            (
                app_guard.current_simulation_idx,
                app_guard.particles.clone(),
            )
        };
        if current_simulation_idx != lua_app.current_simulation_idx {
            let simulation_path = {
                let app_guard = app_clone.lock().unwrap();
                app_guard
                    .possible_simulations
                    .get(current_simulation_idx)
                    .unwrap()
                    .clone()
            };
            lua_app.switch_simulation(simulation_path, current_simulation_idx, particles)?;
        }

        // Example call later:
        let simulation = lua_app.simulation_instance.as_ref().unwrap();
        let particles: Vec<Vec<f64>> = simulation.call_method("simulate", ())?;

        {
            let mut app_guard = app_clone.lock().unwrap();
            app_guard.particles = particles;
        }

        // Draw the particles
        {
            let mut app_guard = app_clone.lock().unwrap();
            tui_clone
                .lock()
                .unwrap()
                .draw(&mut app_guard)
                .expect("Failed to draw UI");
        }

        // Sleep for a short period to control the simulation speed
        thread::sleep(Duration::from_millis(250));
    }
    // Exit the user interface.
    tui_clone.lock().unwrap().exit()?;
    Ok(())
}
