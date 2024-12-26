use crossterm::event::{self, KeyCode, KeyEvent};

use crate::{
    app::{App, AppResult},
    lua_sim::LuaSim,
};

pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
    lua_sim: &mut LuaSim,
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
            lua_sim.set_texture_index(app.texture_index)?;
        }
        Tab => {
            app.current_simulation_idx =
                (app.current_simulation_idx + 1) % app.possible_simulations.len();
            lua_sim.switch_simulation(app)?;
        }
        _ => {
            lua_sim.handle_key_events(key_event, app)?;
        }
    }

    Ok(())
}
