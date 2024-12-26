use crossterm::event::KeyEvent;
use mlua::{Lua, ObjectLike};

use crate::app::{App, AppResult};

pub struct ShouldUpdate {
    pub simulation: bool,
    pub particles: bool,
    pub params: bool,
    pub config: bool,
}

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

        self.update_all_data(app)?;

        Ok(())
    }

    pub fn switch_simulation(&mut self, app: &mut App) -> AppResult<()> {
        let idx = app.current_simulation_idx;
        app.texture_index = 0;
        self.current_simulation = Lua::new();
        self.current_simulation_idx = idx;
        self.load_simulation(app)?;
        Ok(())
    }

    pub fn get_should_update(update: &mlua::Table) -> AppResult<ShouldUpdate> {
        return Ok(ShouldUpdate {
            particles: update.get::<bool>("particles")?,
            simulation: update.get::<bool>("simulation")?,
            params: update.get::<bool>("params")?,
            config: update.get::<bool>("config")?,
        });
    }

    pub fn handle_key_events(&self, key_event: KeyEvent, app: &mut App) -> AppResult<()> {
        let lua = &self.current_simulation;
        let key = match key_event.code {
            crossterm::event::KeyCode::Char(c) => c.to_string(),
            _ => format!("{:?}", key_event.code),
        };

        let key_table = lua.create_table()?;
        key_table.set("code", format!("{}", key))?;
        key_table.set("modifiers", self.format_modifiers(key_event.modifiers)?)?;
        key_table.set("kind", format!("{:?}", key_event.kind))?;

        let update: mlua::Table = self
            .simulation_instance
            .as_ref()
            .unwrap()
            .call_method("handle_key_events", key_table)?;

        app.hande_update(&update, self)?;

        Ok(())
    }

    pub fn update_all_data(&self, app: &mut App) -> AppResult<()> {
        self.update_params(app)?;
        self.update_textures(app)?;
        self.update_config(app)?;
        Ok(())
    }

    pub fn update_particles(&self, app: &mut App) -> AppResult<()> {
        let sim = self.simulation_instance.as_ref().unwrap();

        // get particles
        app.particles = sim.call_method("get_particles", ())?;
        Ok(())
    }

    pub fn update_params(&self, app: &mut App) -> AppResult<()> {
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

    pub fn update_textures(&self, app: &mut App) -> AppResult<()> {
        let sim = self.simulation_instance.as_ref().unwrap();

        // get textures
        let styles: Vec<Vec<String>> = sim.call_method("get_textures", ())?;

        app.textures = styles
            .iter()
            .map(|style| style.iter().map(|s| s.chars().next().unwrap()).collect())
            .collect();
        Ok(())
    }
    pub fn update_config(&self, app: &mut App) -> AppResult<()> {
        let sim = self.simulation_instance.as_ref().unwrap();

        // get config
        let config_table: mlua::Table = sim.call_method("get_config", ())?;

        app.color_enabled = config_table.get::<bool>("color_enabled")?;
        app.mill_per_frame = config_table.get::<u64>("mill_per_frame")?;

        Ok(())
    }

    pub fn set_texture_index(&self, idx: usize) -> AppResult<()> {
        let sim = self.simulation_instance.as_ref().unwrap();
        let _: () = sim.call_method("set_texture_index", idx)?;
        Ok(())
    }

    fn format_modifiers(
        &self,
        modifiers: crossterm::event::KeyModifiers,
    ) -> AppResult<mlua::Table> {
        let lua = &self.current_simulation;
        let mods = lua.create_table()?;
        mods.set(
            "shift",
            modifiers.contains(crossterm::event::KeyModifiers::SHIFT),
        )?;
        mods.set(
            "ctrl",
            modifiers.contains(crossterm::event::KeyModifiers::CONTROL),
        )?;
        mods.set(
            "alt",
            modifiers.contains(crossterm::event::KeyModifiers::ALT),
        )?;
        mods.set(
            "super",
            modifiers.contains(crossterm::event::KeyModifiers::SUPER),
        )?;
        mods.set(
            "hyper",
            modifiers.contains(crossterm::event::KeyModifiers::HYPER),
        )?;
        mods.set(
            "meta",
            modifiers.contains(crossterm::event::KeyModifiers::META),
        )?;
        Ok(mods)
    }
}
