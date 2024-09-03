use ratatui::{
    layout::Rect,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
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
            let particle_brightness = rand::thread_rng()
                .gen_range(app.min_brightness..app.max_brightness)
                * app.noise_intensity;

            let idx = (particle_brightness * 4.0).clamp(0.0, 3.0) as usize;

            particles.push(PARTICLES[idx]);
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
            app.noise_intensity, app.min_brightness, app.max_brightness,
        ))
        .block(block);
        f.render_widget(info, centered_area(23, 5, f_area));
    }
}

fn centered_area(width: usize, height: usize, area: Rect) -> Rect {
    let x = (area.width.saturating_sub(width as u16)) / 2;
    let y = (area.height.saturating_sub(height as u16)) / 2;

    Rect::new(x, y, width as u16, height as u16)
}
