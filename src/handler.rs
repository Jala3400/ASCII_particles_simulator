use crossterm::event::{self, KeyCode, KeyEvent};

use crate::app::App;

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) {
    if key_event.kind == event::KeyEventKind::Release {
        return;
    }
    use KeyCode::*;
    match key_event.code {
        Char('q') => app.quit(),
        // Char('n') => app.current_screen = CurrentScreen::Noise,
        // Char('f') => app.current_screen = CurrentScreen::Fire,
        Char('i') => app.show_info = !app.show_info,
        Tab => {
            app.particles_index = (app.particles_index + 1) % app.particles_styles.len();
        }
        _ => {
            // app.show_info = true;
            // match app.current_screen {
            //     CurrentScreen::Noise => return handle_noise_key(key_event, app),
            //     CurrentScreen::Fire => return handle_fire_key(key_event, app),
            // }
        }
    }

    app.show_info = false;
}
