use std::{collections::HashMap, error};

use mlua::FromLua;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Clone, FromLua)]
pub struct ShouldUpdate {
    pub simulation: bool,
    pub particles: bool,
    pub params: bool,
    pub config: bool,
}

pub struct App {
    pub running: bool,
    pub particles: Vec<Vec<f64>>,
    pub show_info: bool,
    pub texture_index: usize,
    pub textures: Vec<Vec<char>>,
    pub possible_simulations: Vec<String>,
    pub current_simulation_idx: usize,
    pub current_params: HashMap<String, f64>,
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
            current_params: HashMap::new(),
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
}
