use crossterm::event::{self, KeyCode, KeyEvent};

use crate::{
    app::{App, CurrentScreen},
    simulations::{fire::FireData, noise::NoiseData},
};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> bool {
    if key_event.kind == event::KeyEventKind::Release {
        return false;
    }
    use KeyCode::*;
    match key_event.code {
        Char('q') => app.quit(),
        Char('n') => app.current_screen = CurrentScreen::Noise,
        Char('f') => app.current_screen = CurrentScreen::Fire,
        Char('i') => app.show_info = !app.show_info,
        Tab => {
            app.particles_index = (app.particles_index + 1) % app.particles_styles.len();
        }
        _ => {
            app.show_info = true;
            match app.current_screen {
                CurrentScreen::Noise => return handle_noise_key(key_event, app),
                CurrentScreen::Fire => return handle_fire_key(key_event, app),
            }
        }
    }

    app.show_info = false;

    true
}

fn handle_noise_key(key: KeyEvent, app: &mut App) -> bool {
    use KeyCode::*;
    match key.code {
        Char('+') => {
            app.noise_data.noise_intensity += 0.1;
        }
        Char('-') => {
            app.noise_data.noise_intensity -= 0.1;
        }
        Up => {
            app.noise_data.min_brightness += 0.1;
            app.noise_data.max_brightness += 0.1;
        }
        Down => {
            app.noise_data.min_brightness -= 0.1;
            app.noise_data.max_brightness -= 0.1;
        }
        Right => {
            app.noise_data.min_brightness -= 0.1;
            app.noise_data.max_brightness += 0.1;
        }
        Left => {
            app.noise_data.min_brightness += 0.1;
            app.noise_data.max_brightness -= 0.1;
            if app.noise_data.min_brightness > app.noise_data.max_brightness {
                let tmp = app.noise_data.min_brightness;
                app.noise_data.min_brightness = app.noise_data.max_brightness;
                app.noise_data.max_brightness = tmp;
            }
        }
        Char('r') => {
            let new_noise_info = NoiseData::new();
            app.noise_data = new_noise_info;
        }
        _ => {
            app.show_info = false;
        }
    }

    true
}

fn handle_fire_key(key: KeyEvent, app: &mut App) -> bool {
    use KeyCode::*;
    match key.code {
        Char('+') => {
            app.fire_data.noise_intensity += 0.01;
        }
        Char('-') => {
            app.fire_data.noise_intensity -= 0.01;
        }
        Up => {
            app.fire_data.below_intensity += 0.01;
        }
        Down => {
            app.fire_data.below_intensity -= 0.01;
        }
        Right => {
            app.fire_data.past_intensity += 0.01;
        }
        Left => {
            app.fire_data.past_intensity -= 0.01;
        }
        Char('.') => {
            app.fire_data.fire_intensity += 0.01;
        }
        Char(',') => {
            app.fire_data.fire_intensity -= 0.01;
        }
        Char('r') => {
            let new_fire_data = FireData::new();
            app.fire_data = new_fire_data;
        }
        _ => {
            app.show_info = false;
        }
    }

    true
}
