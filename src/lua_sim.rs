use crossterm::event::KeyEvent;
use mlua::{Lua, ObjectLike};

use crate::app::{App, AppResult};

pub struct LuaSim {
    pub current_simulation: Lua,                  // Store the Lua instance
    pub simulation_instance: Option<mlua::Table>, // Store the simulation instance
    pub current_simulation_idx: usize,
}

impl LuaSim {
    pub fn new() -> Self {
        LuaSim {
            current_simulation: Lua::new(),
            simulation_instance: None,
            current_simulation_idx: 0,
        }
    }

    pub fn load_simulation(&mut self, app: &mut App) -> AppResult<()> {
        let path = &app.possible_simulations[app.current_simulation_idx];
        let particles = app.particles.clone();
        let content = std::fs::read_to_string(path)?;
        let chunk = self.current_simulation.load(&content);
        chunk.exec()?;

        let instance: mlua::Table = self.current_simulation.globals().get("Simulation")?;

        // Call methods on the instance
        let init_func: mlua::Function = instance.get("setup")?;
        let sim: mlua::Table = init_func.call(particles)?;

        self.simulation_instance = Some(sim);

        self.update_all_params(app)?;

        Ok(())
    }

    pub fn switch_simulation(&mut self, app: &mut App) -> AppResult<()> {
        let idx = app.current_simulation_idx;
        self.current_simulation = Lua::new();
        self.current_simulation_idx = idx;
        self.load_simulation(app)?;
        Ok(())
    }

    pub fn handle_key_events(&mut self, key_event: KeyEvent, app: &mut App) -> AppResult<()> {
        let lua = &self.current_simulation;
        let key = match key_event.code {
            crossterm::event::KeyCode::Char(c) => c.to_string(),
            _ => format!("{:?}", key_event.code),
        };

        let key_table = lua.create_table()?;
        key_table.set("code", format!("{}", key))?;
        key_table.set("modifiers", format!("{}", key_event.modifiers))?;
        key_table.set("kind", format!("{:?}", key_event.kind))?;

        let _: () = self
            .simulation_instance
            .as_ref()
            .unwrap()
            .call_method("handle_key_events", key_table)?;

        self.update_all_params(app)?;

        Ok(())
    }

    pub fn update_all_params(&mut self, app: &mut App) -> AppResult<()> {
        self.update_params(app)?;
        self.update_textures(app)?;
        Ok(())
    }

    pub fn update_params(&mut self, app: &mut App) -> AppResult<()> {
        let sim = self.simulation_instance.as_ref().unwrap();

        // get params
        let params_array: mlua::Table = sim.call_method("get_params", ())?;

        app.current_params.clear();
        for pair in params_array.pairs::<i32, mlua::Table>() {
            let (_, param) = pair?;
            let name: String = param.get("name")?;
            let value: f64 = param.get("value")?;
            app.current_params.insert(name, value);
        }
        Ok(())
    }

    pub fn update_textures(&mut self, app: &mut App) -> AppResult<()> {
        let sim = self.simulation_instance.as_ref().unwrap();

        // get textures
        let styles: Vec<Vec<String>> = sim.call_method("get_textures", ())?;

        app.textures = styles
            .iter()
            .map(|style| style.iter().map(|s| s.chars().next().unwrap()).collect())
            .collect();
        Ok(())
    }
}
