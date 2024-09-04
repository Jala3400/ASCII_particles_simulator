use rand::Rng;
use ratatui::{
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::{app::App, ui::centered_area};

#[derive(Clone)]
pub struct FireData {
    pub particles: Vec<Vec<f64>>,
    pub noise_intensity: f64,
    pub fire_intensity: f64,
    pub past_intensity: f64,
    pub below_intensity: f64,
}
impl FireData {
    pub fn new() -> Self {
        let mut particles = vec![vec![0.0; 100]; 100];
        particles[99] = vec![1.0; 100];

        FireData {
            particles,
            noise_intensity: 0.07,
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
pub fn draw_fire(f: &mut Frame, app: &mut App) {
    let f_area = f.area();
    f.render_widget(Clear, f_area); //this clears the entire screen and anything already drawn

    let mut particles: String = String::new();

    let height = f_area.height;
    let width = f_area.width;

    if height != app.fire_data.particles.len() as u16
        || width != app.fire_data.particles[0].len() as u16
    {
        app.fire_data
            .change_dimensions(width as usize, height as usize);
    }

    for i in 0..height {
        for j in 0..width {
            let past_brightness =
                app.fire_data.particles[i as usize][j as usize] * app.fire_data.past_intensity;
            let below_brightness = if i < f_area.height - 1 {
                app.fire_data.particles[(i + 1) as usize][j as usize]
            } else {
                1.0
            } * app.fire_data.below_intensity;

            let noise_brightness =
                rand::thread_rng().gen_range(-1.1..1.0) * app.fire_data.noise_intensity;

            let particle_brightness = (past_brightness + below_brightness + noise_brightness)
                * app.fire_data.fire_intensity;

            app.fire_data.particles[i as usize][j as usize] = particle_brightness;

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
            "Noise intensity: {}     \nFire intensity: {}     \nPast intensity: {}     \nBelow intensity: {}     ",
            app.fire_data.noise_intensity,
            app.fire_data.fire_intensity,
            app.fire_data.past_intensity,
            app.fire_data.below_intensity,
        ))
        .block(block);
        f.render_widget(info, centered_area(25, 6, f_area));
    }
}
