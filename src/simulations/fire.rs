use ratatui::{
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::{app::App, ui::centered_area};

pub struct FireData {
    pub particles: Vec<Vec<f32>>,
    pub fire_intensity: f32,
    pub past_intensity: f32,
    pub below_intensity: f32,
}
impl FireData {
    pub fn new() -> Self {
        let mut particles = vec![vec![0.0; 100]; 100];
        particles[99] = vec![1.0; 100];

        FireData {
            particles,
            fire_intensity: 1.0,
            past_intensity: 0.25,
            below_intensity: 0.5,
        }
    }
}
pub fn draw_fire(f: &mut Frame, app: &App) {
    
}
