use ratatui::{
    layout::Rect,
    text::Text,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &mut App) {
    let f_area = f.area();
    let length = app.textures[app.texture_index].len() as f64;
    let texture = app.textures[app.texture_index].clone();
    let mut particles = String::with_capacity(app.particles.len() * (app.particles[0].len() + 1));

    for row in &app.particles {
        for &particle_brightness in row {
            let idx = (particle_brightness * length).clamp(0.0, length - 1.0) as usize;
            particles.push(texture[idx]);
        }
        particles.push('\n');
    }

    let particles = Text::raw(particles);

    f.render_widget(particles, f_area);

    if app.show_info {
        draw_info(f, app)
    }
}

fn draw_info(f: &mut Frame, app: &mut App) {
    let info_str = app.current_params.clone();

    let (num_lines, max_line_length) = info_str.lines().fold((0, 0), |(count, max_len), line| {
        (count + 1, max_len.max(line.chars().count()))
    });

    let max_width = f.area().width.saturating_sub(2);
    let max_height = f.area().height.saturating_sub(2);
    let width = max_width.min(50.max(max_line_length as u16));
    let height = max_height.min((num_lines + 2) as u16);
    let area = centered_area(width as usize, height as usize, f.area());
    let info = Paragraph::new(info_str).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    f.render_widget(Clear, area);
    f.render_widget(info, area);
}

pub fn centered_area(width: usize, height: usize, area: Rect) -> Rect {
    let x = (area.width.saturating_sub(width as u16)) / 2;
    let y = (area.height.saturating_sub(height as u16)) / 2;

    Rect::new(x, y, width as u16, height as u16)
}
