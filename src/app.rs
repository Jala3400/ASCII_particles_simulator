use std::error;

use crate::simulations::{fire::FireData, noise::NoiseData};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Clone)]
pub struct App {
    pub running: bool,
    pub particles: Vec<Vec<f64>>,
    pub current_screen: CurrentScreen,
    pub noise_data: NoiseData,
    pub fire_data: FireData,
    pub show_info: bool,
    pub particles_index: usize,
    pub particles_styles: [[char; 4]; 2],
}

#[derive(Clone, PartialEq)]
pub enum CurrentScreen {
    Noise,
    Fire,
}

impl App {
    pub fn new() -> Self {
        App {
            running: true,
            particles: vec![vec![0.0; 100]; 100],
            current_screen: CurrentScreen::Noise,
            noise_data: NoiseData::new(),
            fire_data: FireData::new(),
            show_info: false,
            particles_index: 0,
            particles_styles: [[' ', '·', '+', '#'], [' ', '.', 'o', '@']],
            // Todo:  .:-=+*#%@  ▁▂▃▄▅▆▇█ ░▒▓█
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
