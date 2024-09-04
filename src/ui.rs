use ratatui::{layout::Rect, Frame};

use crate::{
    app::{App, CurrentScreen},
    simulations::{fire, noise},
};

pub fn render(f: &mut Frame, app: &mut App) {
    match app.current_screen {
        CurrentScreen::Noise => noise::draw_noise(f, app),
        CurrentScreen::Fire => fire::draw_fire(f, app),
    }
}

pub fn centered_area(width: usize, height: usize, area: Rect) -> Rect {
    let x = (area.width.saturating_sub(width as u16)) / 2;
    let y = (area.height.saturating_sub(height as u16)) / 2;

    Rect::new(x, y, width as u16, height as u16)
}
