use app::{App, AppResult};
use ascii_particles_simulator::{
    app::{self, ShouldUpdate},
    handler,
    lua_sim::LuaSim,
    tui,
};
use crossterm::event::{self, Event};
use handler::handle_key_events;
use mlua::ObjectLike;
use ratatui::prelude::*;
use std::{io, time::Duration};
use tui::Tui;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Load the simulation files.
    load_files(&mut app)?;

    let mut tui = init_terminal(&mut app)?;
    tui.init()?;

    let terminal_size = tui.terminal.size()?;
    let width = terminal_size.width as usize;
    let height = terminal_size.height as usize;
    app.change_dimensions(width, height);

    let mut lua_sim = LuaSim::new();

    // Set up the initial simulation
    let noise_idx = app
        .possible_simulations
        .iter()
        .position(|x| x.contains("noise"))
        .unwrap_or(0);
    lua_sim.current_simulation_idx = noise_idx;
    app.current_simulation_idx = noise_idx;

    lua_sim.load_simulation(&mut app)?;

    while app.running {
        let sim = lua_sim.simulation_instance.as_ref().unwrap();

        // Check if the terminal size has changed
        check_size(&mut app, sim, &tui)?;

        // Get the new particles
        let update: mlua::Table = sim.call_method("simulate", ())?;
        let should_update = ShouldUpdate {
            particles: update.get::<bool>("particles")?,
            simulation: update.get::<bool>("simulation")?,
            params: update.get::<bool>("params")?,
            config: update.get::<bool>("config")?,
        };

        if should_update.particles {
            let particles: Vec<Vec<f64>> = sim.call_method("get_particles", ())?;
            app.particles = particles;
        }
        tui.draw(&mut app).expect("Failed to draw UI");

        let frame_duration = Duration::from_millis(250);
        let frame_start = std::time::Instant::now();

        let remaining_time = frame_duration
            .checked_sub(frame_start.elapsed())
            .unwrap_or(Duration::from_millis(0));

        // Handle events until the frame duration is up
        while frame_start.elapsed() < frame_duration {
            if event::poll(remaining_time)? {
                let event = event::read()?;
                match event {
                    Event::Key(event) => {
                        handle_key_events(event, &mut app, &mut lua_sim)?;
                    }
                    _ => {}
                }
                tui.draw(&mut app).expect("Failed to draw UI");
                // tx.send(event).unwrap();
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

fn load_files(app: &mut App) -> AppResult<()> {
    // Get all simulation files from the simulations directory
    let simulation_files = std::fs::read_dir("simulations_lua")?
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

    app.possible_simulations = simulation_files;
    Ok(())
}

fn init_terminal(app: &mut App) -> AppResult<Tui<CrosstermBackend<io::Stdout>>> {
    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;

    // Get the size of the terminal screen
    let terminal_size = terminal.size()?;
    let screen_width = terminal_size.width as usize;
    let screen_height = terminal_size.height as usize;
    app.change_dimensions(screen_width, screen_height);

    Ok(Tui::new(terminal))
}

fn check_size(
    app: &mut App,
    simulation: &mlua::Table,
    tui: &Tui<CrosstermBackend<io::Stdout>>,
) -> AppResult<()> {
    let current_height = app.particles.len();
    let current_width = app.particles[0].len();

    let terminal_size = tui.terminal.size()?;
    let width = terminal_size.width as usize;
    let height = terminal_size.height as usize;
    if width != current_width || height != current_height {
        app.change_dimensions(width as usize, height as usize);
        let _: () = simulation.call_method("set_particles", app.particles.clone())?;
    }

    Ok(())
}
