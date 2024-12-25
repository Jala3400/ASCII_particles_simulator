use crossterm::event::{self, KeyCode, KeyEvent};

use crate::{app::{App, AppResult}, lua_sim::LuaSim};

pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
    lua_app: &mut LuaSim,
) -> AppResult<()> {
    if key_event.kind == event::KeyEventKind::Release {
        return Ok(());
    }
    use KeyCode::*;
    match key_event.code {
        Char('q') => {
            app.quit();
        }
        Char('i') => {
            app.show_info = !app.show_info;
        }
        Enter => {
            app.texture_index = (app.texture_index + 1) % app.textures.len();
        }
        Tab => {
            app.current_simulation_idx =
                (app.current_simulation_idx + 1) % app.possible_simulations.len();
            lua_app.switch_simulation(app)?;
        }
        _ => {
            lua_app.handle_key_events(key_event, app)?;
        }
    }

    Ok(())
}
