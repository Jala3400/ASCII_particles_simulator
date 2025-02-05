use crossterm::event::{Event, KeyEvent, MouseEvent};
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

    pub fn handle_events(&self, event: &Event, app: &mut App) -> AppResult<()> {
        let update;
        let event_table = match event {
            Event::FocusGained => {
                let table = self.current_simulation.create_table()?;
                table.set("type", "FocusGained")?;
                table
            }
            Event::FocusLost => {
                let table = self.current_simulation.create_table()?;
                table.set("type", "FocusLost")?;
                table
            }
            Event::Key(key_event) => self.format_key_events(key_event)?,
            Event::Mouse(mouse_event) => self.format_mouse_events(mouse_event)?,
            Event::Paste(paste_event) => self.format_paste_events(paste_event)?,
            Event::Resize(x, y) => self.format_resize_events(*x, *y)?,
        };

        update = self
            .simulation_instance
            .as_ref()
            .ok_or("Simulation instance not initialized")?
            .call_method("handle_events", event_table)?;

        if let Some(update) = update {
            app.handle_update(&update, self)?;
        }

        Ok(())
    }

    fn format_key_events(&self, key_event: &KeyEvent) -> AppResult<mlua::Table> {
        let lua = &self.current_simulation;
        let key = match key_event.code {
            crossterm::event::KeyCode::Char(c) => c.to_string(),
            _ => format!("{:?}", key_event.code),
        };

        let key_table = lua.create_table_with_capacity(4, 0)?;
        key_table.raw_set("type", "Key")?;
        key_table.raw_set("code", format!("{}", key))?;
        key_table.raw_set("modifiers", self.format_modifiers(key_event.modifiers)?)?;
        key_table.raw_set("kind", format!("{:?}", key_event.kind))?;

        Ok(key_table)
    }

    fn format_mouse_events(&self, mouse_event: &MouseEvent) -> AppResult<mlua::Table> {
        let lua = &self.current_simulation;
        let (kind, button): (&str, &str) = match mouse_event.kind {
            crossterm::event::MouseEventKind::Down(button) => ("Down", &format!("{:?}", button)),
            crossterm::event::MouseEventKind::Up(button) => ("Up", &format!("{:?}", button)),
            crossterm::event::MouseEventKind::Drag(button) => ("Drag", &format!("{:?}", button)),
            _ => (&format!("{:?}", mouse_event.kind), "None"),
        };
        let mouse_table = lua.create_table_with_capacity(6, 0)?;
        mouse_table.raw_set("type", "Mouse")?;
        mouse_table.raw_set("x", mouse_event.column)?;
        mouse_table.raw_set("y", mouse_event.row)?;
        mouse_table.raw_set("kind", kind)?;
        mouse_table.raw_set("button", button)?;
        mouse_table.raw_set("modifiers", self.format_modifiers(mouse_event.modifiers)?)?;
        Ok(mouse_table)
    }

    fn format_paste_events(&self, paste_event: &String) -> AppResult<mlua::Table> {
        let lua = &self.current_simulation;
        let paste_table = lua.create_table()?;
        paste_table.set("type", "Paste")?;
        paste_table.set("text", paste_event.clone())?;
        Ok(paste_table)
    }

    fn format_resize_events(&self, x: u16, y: u16) -> AppResult<mlua::Table> {
        let lua = &self.current_simulation;
        let resize_table = lua.create_table_with_capacity(3, 0)?;
        resize_table.raw_set("type", "Resize")?;
        resize_table.raw_set("x", x)?;
        resize_table.raw_set("y", y)?;
        Ok(resize_table)
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
        app.current_params = sim.call_method("get_params", ())?;

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

        app.millis_per_frame = config_table.get::<u64>("millis_per_frame")?;
        app.texture_index = config_table.get::<usize>("texture_index")?;

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
        let mods = lua.create_table_with_capacity(6, 0)?;
        mods.raw_set(
            "Shift",
            modifiers.contains(crossterm::event::KeyModifiers::SHIFT),
        )?;
        mods.raw_set(
            "Ctrl",
            modifiers.contains(crossterm::event::KeyModifiers::CONTROL),
        )?;
        mods.raw_set(
            "Alt",
            modifiers.contains(crossterm::event::KeyModifiers::ALT),
        )?;
        mods.raw_set(
            "Super",
            modifiers.contains(crossterm::event::KeyModifiers::SUPER),
        )?;
        mods.raw_set(
            "Hyper",
            modifiers.contains(crossterm::event::KeyModifiers::HYPER),
        )?;
        mods.raw_set(
            "Meta",
            modifiers.contains(crossterm::event::KeyModifiers::META),
        )?;
        Ok(mods)
    }
}
