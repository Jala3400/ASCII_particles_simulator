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
        noise_intensity = 0.07,
        fire_intensity = 1.0,
        past_intensity = 0.25,
        below_intensity = 0.70
    }
    return self
end

function Simulation:simulate()
    local noise_intensity = self.params.noise_intensity
    local fire_intensity = self.params.fire_intensity
    local past_intensity = self.params.past_intensity
    local below_intensity = self.params.below_intensity

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
            local past_brightness = self.particles[i][j] * past_intensity

            local below_brightness = 1.0
            if i < #self.particles then
                below_brightness = self.particles[i + 1][j]
            end
            below_brightness = below_brightness * below_intensity

            local noise_brightness = (math.random() * 2.0 - 1.0) * noise_intensity

            local particle_brightness = (past_brightness + below_brightness + noise_brightness)
                * fire_intensity

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

function Simulation:handle_key_events(key)
    local should_update = {
        particles = false,
        simulation = false,
        params = false,
        config = false,
    }

    local key_actions = {
        ['+'] = function() self.params.noise_intensity = self.params.noise_intensity + 0.01 end,
        ['-'] = function() self.params.noise_intensity = self.params.noise_intensity - 0.01 end,
        ['Up'] = function() self.params.below_intensity = self.params.below_intensity + 0.01 end,
        ['Down'] = function() self.params.below_intensity = self.params.below_intensity - 0.01 end,
        ['Right'] = function() self.params.past_intensity = self.params.past_intensity + 0.01 end,
        ['Left'] = function() self.params.past_intensity = self.params.past_intensity - 0.01 end,
        ['.'] = function() self.params.fire_intensity = self.params.fire_intensity + 0.01 end,
        [','] = function() self.params.fire_intensity = self.params.fire_intensity - 0.01 end,
        ['r'] = function()
            self.params = {
                noise_intensity = 0.07,
                fire_intensity = 1.0,
                past_intensity = 0.25,
                below_intensity = 0.70
            }
        end
    }

    local key = key.code
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
            if self.particles[i] and self.particles[i][j] then
                result[i][j] = self.particles[i][j]
            else
                local base_brightness = (i / event.y) * 0.8
                local random_variation = (math.random() * 0.2) - 0.1
                result[i][j] = math.max(0, math.min(1, base_brightness + random_variation))
            end
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
Noise intensity: %.2f
Fire intensity: %.2f
Past intensity: %.2f
Below intensity: %.2f
]],
        self.params.noise_intensity,
        self.params.fire_intensity,
        self.params.past_intensity,
        self.params.below_intensity
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
