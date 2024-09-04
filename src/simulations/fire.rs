use rand::Rng;
use ratatui::{
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::{app::App, ui::centered_area};

#[derive(Clone)]
pub struct FireData {
    pub particles: Vec<Vec<f32>>,
    pub noise_intensity: f32,
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
            noise_intensity: 0.10,
            fire_intensity: 1.0,
            past_intensity: 0.25,
            below_intensity: 0.70,
        }
    }

    pub fn change_dimensions(&mut self, width: usize, height: usize) {
        self.particles = vec![vec![0.0; width]; height];
        self.particles[height - 1] = vec![1.0; width];
    }
}
pub fn draw_fire(f: &mut Frame, app: &mut App) {}
