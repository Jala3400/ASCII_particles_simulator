use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::{
    app::{App, AppResult},
    lua_sim::LuaSim,
};

pub fn handle_events(event: &Event, app: &mut App, lua_sim: &mut LuaSim) -> AppResult<()> {
    if let Event::Key(key_event) = event {
        handle_key_events(key_event, app, lua_sim)?;
    }
    lua_sim.handle_events(event, app)?;

    Ok(())
}

pub fn handle_key_events(event: &KeyEvent, app: &mut App, lua_sim: &mut LuaSim) -> AppResult<()> {
    if event.kind == event::KeyEventKind::Release {
        return Ok(());
    }

    use KeyCode::*;
    match event.code {
        Char('q') => {
            app.quit();
        }
        Char('i') => {
            app.show_info = !app.show_info;
        }
        Enter => {
            if event.modifiers.contains(event::KeyModifiers::SHIFT) {
                app.texture_index = (app.texture_index - 1) % app.textures.len();
                lua_sim.set_texture_index(app.texture_index)?;
            } else {
                app.texture_index = (app.texture_index + 1) % app.textures.len();
                lua_sim.set_texture_index(app.texture_index)?;
            }
        }
        Tab => {
            if event.modifiers.contains(event::KeyModifiers::SHIFT) {
                app.current_simulation_idx =
                    (app.current_simulation_idx - 1) % app.possible_simulations.len();
                lua_sim.switch_simulation(app)?;
            } else {
                app.current_simulation_idx =
                    (app.current_simulation_idx + 1) % app.possible_simulations.len();
                lua_sim.switch_simulation(app)?;
            }
        }
        _ => {}
    }

    Ok(())
}
