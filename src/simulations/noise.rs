use rand::Rng;
use ratatui::{
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::{app::App, ui::centered_area};

#[derive(Clone)]
pub struct NoiseData {
    pub noise_intensity: f64,
    pub min_brightness: f64,
    pub max_brightness: f64,
}

impl NoiseData {
    pub fn new() -> Self {
        NoiseData {
            noise_intensity: 1.0,
            min_brightness: -1.0,
            max_brightness: 1.0,
        }
    }
}

pub fn draw_noise(f: &mut Frame, app: &mut App) {
    let f_area = f.area();
    f.render_widget(Clear, f_area); //this clears the entire screen and anything already drawn

    let mut particles: String = String::new();

    for _ in 0..f_area.height {
        for _ in 0..f_area.width {
            let particle_brightness = rand::thread_rng()
                .gen_range(app.noise_data.min_brightness..app.noise_data.max_brightness)
                * app.noise_data.noise_intensity;

            let idx = (particle_brightness * 4.0).clamp(0.0, 3.0) as usize;

            particles.push(app.particles_styles[app.particles_index][idx]);
        }
        particles.push('\n');
    }

    let particles = Paragraph::new(particles);

    f.render_widget(particles, f_area);

    if app.show_info {
        let block = Block::default()
            .title("Info")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let info = Paragraph::new(format!(
            "Noise intensity: {}     \nMin brightness: {}     \nMax brightness: {}     ",
            app.noise_data.noise_intensity,
            app.noise_data.min_brightness,
            app.noise_data.max_brightness,
        ))
        .block(block);
        f.render_widget(info, centered_area(25, 5, f_area));
    }
}
