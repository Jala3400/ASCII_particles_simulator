use std::sync::RwLockWriteGuard;

use crossterm::event::{self, KeyCode, KeyEvent};

use crate::app::{App, AppResult, CurrentScreen};

pub fn handle_key_events(key_event: KeyEvent, mut app: RwLockWriteGuard<App>) -> AppResult<()> {
    if key_event.kind == event::KeyEventKind::Release {
        return Ok(());
    }
    use KeyCode::*;
    match key_event.code {
        Char('q') => app.quit(),
        Char('n') => app.current_screen = CurrentScreen::Noise,
        Char('f') => app.current_screen = CurrentScreen::Fire,
        Char('i') => app.show_info = !app.show_info,
        Char('r') => {
            let new_app = App::new();
            *app = new_app;
        }
        Tab => {
            app.particles_index = (app.particles_index + 1) % app.particles_styles.len();
        }
        _ => match app.current_screen {
            CurrentScreen::Noise => handle_noise_key(key_event, app),
            CurrentScreen::Fire => handle_fire_key(key_event, app),
        },
    }

    Ok(())
}

fn handle_noise_key(key: KeyEvent, mut app: RwLockWriteGuard<App>) {
    use KeyCode::*;
    match key.code {
        Char('+') => {
            app.noise_data.noise_intensity += 0.1;
            app.show_info = true;
        }
        Char('-') => {
            app.noise_data.noise_intensity -= 0.1;
            app.show_info = true;
        }
        Up => {
            app.noise_data.min_brightness += 0.1;
            app.noise_data.max_brightness += 0.1;
            app.show_info = true;
        }
        Down => {
            app.noise_data.min_brightness -= 0.1;
            app.noise_data.max_brightness -= 0.1;
            app.show_info = true;
        }
        Right => {
            app.noise_data.min_brightness -= 0.1;
            app.noise_data.max_brightness += 0.1;
            app.show_info = true;
        }
        Left => {
            app.noise_data.min_brightness += 0.1;
            app.noise_data.max_brightness -= 0.1;
            if app.noise_data.min_brightness > app.noise_data.max_brightness {
                let tmp = app.noise_data.min_brightness;
                app.noise_data.min_brightness = app.noise_data.max_brightness;
                app.noise_data.max_brightness = tmp;
            }
            app.show_info = true;
        }
        Char('r') => {
            let new_app = App::new();
            *app = new_app;
        }
        _ => {
            app.show_info = false;
        }
    }
}

fn handle_fire_key(key: KeyEvent, mut app: RwLockWriteGuard<App>) {
    use KeyCode::*;
    match key.code {
        Char('q') => return,
        _ => {}
    }
}

pub fn handle_resize_event(mut app: RwLockWriteGuard<App>, x: u16, y: u16) -> AppResult<()> {
    match app.current_screen {
        CurrentScreen::Noise => {}
        CurrentScreen::Fire => app.fire_data.change_dimensions(x as usize, y as usize),
    }
    Ok(())
}
