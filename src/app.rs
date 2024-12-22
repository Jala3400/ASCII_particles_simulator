use mlua::{Lua, ObjectLike};
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Clone)]
pub struct App {
    pub running: bool,
    pub particles: Vec<Vec<f64>>,
    pub show_info: bool,
    pub particles_index: usize,
    pub particles_styles: [[char; 4]; 2],
    pub possible_simulations: Vec<String>,
    pub current_simulation_idx: usize,
}

impl App {
    pub fn new() -> Self {
        App {
            running: true,
            particles: vec![vec![0.0; 100]; 100],
            show_info: false,
            particles_index: 0,
            particles_styles: [[' ', '·', '+', '#'], [' ', '.', 'o', '@']],
            // Todo:  .:-=+*#%@  ▁▂▃▄▅▆▇█ ░▒▓█
            possible_simulations: vec!["".to_string()],
            current_simulation_idx: 0,
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn change_dimensions(&mut self, width: usize, height: usize) {
        self.particles = vec![vec![0.0; width]; height];
    }
}

pub struct LuaApp {
    pub current_simulation: Lua,                  // Store the Lua instance
    pub simulation_instance: Option<mlua::Table>, // Store the simulation instance
    pub current_simulation_idx: usize,
}

impl LuaApp {
    pub fn new() -> Self {
        LuaApp {
            current_simulation: Lua::new(),
            simulation_instance: None,
            current_simulation_idx: 0,
        }
    }

    pub fn load_simulation(&mut self, path: &str, particles: Vec<Vec<f64>>) -> AppResult<()> {
        let content = std::fs::read_to_string(path)?;
        let chunk = self.current_simulation.load(&content);
        chunk.exec()?;

        let instance: mlua::Table = self.current_simulation.globals().get("Simulation")?;

        // Call methods on the instance
        let init_func: mlua::Function = instance.get("setup")?;
        let sim: mlua::Table = init_func.call(())?;
        sim.call_method("set_particles", particles)?;
        self.simulation_instance = Some(sim);

        Ok(())
    }

    pub fn switch_simulation(
        &mut self,
        path: String,
        idx: usize,
        particles: Vec<Vec<f64>>,
    ) -> AppResult<()> {
        self.current_simulation = Lua::new();
        self.current_simulation_idx = idx;
        self.load_simulation(&path, particles)?;
        Ok(())
    }
}
