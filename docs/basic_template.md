# Plantilla base

Esto puede servir como punto de inicio más básico.

Tiene los métodos mínimos para funcionar, pero se recomienda mirar el [ejemplo de ruido](noise_lua_example.md) para ver cómo se hacen simulaciones más complejas.

```lua
Simulation = {}
Simulation.__index = Simulation

function Simulation.setup(particles)
    local self = setmetatable({}, Simulation)
    self.particles = particles
    self.textures = { { ' ', '·', '+', '#' }, { ' ', '.', 'o', '@' } }
    self.config = {
        millis_per_frame = 250,
        texture_index = 0, -- starting at 0, because config is handled in rust
    }
    self.params = {
        intensity = 1,
    }
    return self
end

function Simulation:simulate()
    local should_update = {
        particles = false,
        simulation = false,
        params = false,
        config = false,
    }

    local result = {}

    -- Initialize result array with same dimensions
    for i = 1, #self.particles do
        result[i] = {}
        for j = 1, #self.particles[1] do
            local particle_brightness = math.random() * self.params.intensity

            result[i][j] = particle_brightness
        end
    end

    self.particles = result

    should_update.particles = true
    return should_update
end

function Simulation:handle_events(event)
    local should_update = {
        particles = false,
        simulation = false,
        params = false,
        config = false,
    }

    local events = {
        -- ['FocusGained'] = function() return self:handle_focus_gained() end,
        -- ['FocusLost'] = function() return self:handle_focus_lost() end,
        ['Key'] = function() return self:handle_key_events(event) end,
        -- ['Mouse'] = function() return self:handle_mouse_events(event) end,
        -- ['Paste'] = function() return self:handle_paste_events(event) end,
        ['Resize'] = function() return self:handle_resize_events(event) end,
    }

    if events[event.type] then
        should_update = events[event.type]()
    end

    return should_update
end

function Simulation:handle_key_events(key_event)
    local should_update = {
        simulation = false,
        particles = false,
        params = false,
        config = false,
    }

    if key_event.kind == "Release" then return should_update end

    local key_actions = {
        ['+'] = function() self.params.intensity = self.params.intensity + 0.1 end,
        ['-'] = function() self.params.intensity = self.params.intensity - 0.1 end,
    }

    local key = key_event.code
    if key_actions[key] then key_actions[key]() end

    should_update.params = true
    return should_update
end

function Simulation:handle_resize_events(event)
    local should_update = {
        particles = false,
        simulation = false,
        params = false,
        config = false,
    }

    local result = {}
    for i = 1, event.y do
        result[i] = {}
        for j = 1, event.x do
            result[i][j] = 0
        end
    end

    self.particles = result

    should_update.particles = true
    return should_update
end

function Simulation:set_particles(particles)
    self.particles = particles or self.particles
end

function Simulation:get_particles()
    return self.particles
end

function Simulation:get_params()
    return string.format(
        [[
Noise intensity: %.1f
]],
        self.params.intensity
    )
end

function Simulation:set_textures(textures)
    self.textures = textures or self.textures
end

function Simulation:get_textures()
    return self.textures
end

function Simulation:set_texture_index(index)
    self.config.texture_index = index
end

function Simulation:get_texture_index()
    return self.config.texture_index
end

function Simulation:set_config(config)
    self.config = config or self.config
end

function Simulation:get_config()
    return self.config
end
```
