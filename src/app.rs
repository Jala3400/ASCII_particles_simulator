use std::{collections::HashMap, error};

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
    pub current_params: HashMap<String, f64>,
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
            current_params: HashMap::new(),
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
