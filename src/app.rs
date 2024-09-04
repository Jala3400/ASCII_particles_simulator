use std::error;

use crate::simulations::{fire::FireData, noise::NoiseData};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Clone)]
pub struct App {
    pub running: bool,
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
}
