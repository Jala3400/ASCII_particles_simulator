use ratatui::{
    widgets::{Clear, Paragraph},
    Frame,
};

use rand::Rng;

use crate::app::App;

const PARTICLES: [char; 4] = [' ', '·', '+', '#'];

pub fn ui(f: &mut Frame, app: &App) {
    let f_area = f.area();
    f.render_widget(Clear, f_area); //this clears the entire screen and anything already drawn

    let mut particles: String = String::new();

    for _ in 0..f_area.height {
        for _ in 0..f_area.width {
            let particle_brightness = rand::thread_rng().gen_range(app.min_brightness..app.max_brightness) * app.noise_intensity;

            let idx = (particle_brightness * 4.0).clamp(0.0, 3.0) as usize;

            particles.push(PARTICLES[idx]);
        }
        particles.push('\n');
    }

    let particles = Paragraph::new(particles);

    f.render_widget(particles, f_area)
}
