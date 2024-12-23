use crossterm::event::{self, KeyCode, KeyEvent};

use crate::app::{App, AppResult, LuaApp};

pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
    lua_app: &mut LuaApp,
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
            app.particles_index = (app.particles_index + 1) % app.particles_styles.len();
        }
        Tab => {
            app.current_simulation_idx =
                (app.current_simulation_idx + 1) % app.possible_simulations.len();
            lua_app.switch_simulation(app)?;
        }
        _ => {
            app.show_info = false;
        }
    }

    Ok(())
}
