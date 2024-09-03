use crate::simulations::{fire::FireData, noise::NoiseData};

pub struct App {
    pub current_screen: CurrentScreen,
    pub noise_data: NoiseData,
    pub fire_data: FireData,
    pub show_info: bool,
    pub particles_index: usize,
    pub particles_styles: [[char; 4]; 2],
}

#[derive(PartialEq)]
pub enum CurrentScreen {
    Noise,
    Fire,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Noise,
            noise_data: NoiseData::new(),
            fire_data: FireData::new(),
            show_info: false,
            particles_index: 0,
            particles_styles: [[' ', '·', '+', '#'], [' ', '.', 'o', '@']],

            // Todo:  .:-=+*#%@  ▁▂▃▄▅▆▇█ ░▒▓█
        }
    }
}
