use std::error;

use mlua::ObjectLike;

use crate::lua_sim::{LuaSim, ShouldUpdate};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    pub running: bool,
    pub particles: Vec<Vec<f64>>,
    pub show_info: bool,
    pub texture_index: usize,
    pub textures: Vec<Vec<char>>,
    pub possible_simulations: Vec<String>,
    pub current_simulation_idx: usize,
    pub current_params: String,
    pub color_enabled: bool,
    pub mill_per_frame: u64,
}

impl App {
    pub fn new() -> Self {
        App {
            running: true,
            particles: vec![vec![0.0; 1]; 1],
            show_info: false,
            texture_index: 0,
            textures: vec![],
            // Todo:  .:-=+*#%@  ▁▂▃▄▅▆▇█ ░▒▓█
            possible_simulations: vec!["".to_string()],
            current_simulation_idx: 0,
            current_params: "".to_string(),
            color_enabled: false,
            mill_per_frame: 250,
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn change_dimensions(&mut self, width: usize, height: usize) {
        self.particles = vec![vec![0.0; width]; height];
    }

    pub fn hande_update(&mut self, update: &mlua::Table, lua_sim: &LuaSim) -> AppResult<()> {
        let should_update: ShouldUpdate = LuaSim::get_should_update(update)?;

        if should_update.simulation {
            let sim = lua_sim.simulation_instance.as_ref().unwrap();
            let update: mlua::Table = sim.call_method("simulate", ())?;
            self.hande_update(&update, lua_sim)?;
        }

        if should_update.particles {
            lua_sim.update_particles(self)?;
        }

        if should_update.params {
            lua_sim.update_params(self)?;
        }

        if should_update.config {
            lua_sim.update_config(self)?;
        }
        Ok(())
    }
}
