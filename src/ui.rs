use ratatui::{
    layout::Rect,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;

// pub fn render(f: &mut Frame, app: &mut App) {
// match app.current_screen {
//     CurrentScreen::Noise => noise::draw_noise(f, app),
//     CurrentScreen::Fire => fire::draw_fire(f, app),
// }
// }

pub fn render(f: &mut Frame, app: &mut App) {
    let f_area = f.area();
    f.render_widget(Clear, f_area); //this clears the entire screen and anything already drawn

    let mut particles: String = String::new();

    for i in 0..app.particles.len() {
        for j in 0..app.particles[0].len() {
            let particle_brightness = app.particles[i][j];

            let idx = (particle_brightness * 4.0).clamp(0.0, 3.0) as usize;

            particles.push(app.particles_styles[app.particles_index][idx]);
        }
        particles.push('\n');
    }

    let particles = Paragraph::new(particles);

    f.render_widget(particles, f_area);

    if app.show_info {
        draw_info(f, app)
    }
}

fn draw_info(f: &mut Frame, app: &mut App) {
    let info_str = format!(
        "{}\n",
        app.current_params
            .iter()
            .map(|(k, v)| format!(" {}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\n")
    );
    let area = centered_area(50, app.current_params.len() + 2, f.area());
    let info = Paragraph::new(info_str).block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    f.render_widget(Clear, area);
    f.render_widget(info, area);
}

pub fn centered_area(width: usize, height: usize, area: Rect) -> Rect {
    let x = (area.width.saturating_sub(width as u16)) / 2;
    let y = (area.height.saturating_sub(height as u16)) / 2;

    Rect::new(x, y, width as u16, height as u16)
}
