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
        noise_intensity = 1,
        min_brightness = 0.0,
        max_brightness = 1.0,
    }
    return self
end

function Simulation:simulate()
    local noise_intensity = self.params.noise_intensity
    local min_brightness = self.params.min_brightness
    local max_brightness = self.params.max_brightness

    local should_update = {
        simulation = false,
        particles = false,
        params = false,
        config = false,
    }

    local result = {}

    -- Initialize result array with same dimensions
    for i = 1, #self.particles do
        result[i] = {}
        for j = 1, #self.particles[1] do
            local particle_brightness = (min_brightness + math.random() *
                (max_brightness - min_brightness)) * noise_intensity

            result[i][j] = particle_brightness
        end
    end

    self.particles = result

    should_update.particles = true
    return should_update
end

function Simulation:handle_events(event)
    local should_update = {
        simulation = false,
        particles = false,
        params = false,
        config = false,
    }

    local events = {
        -- ['FocusGained'] = function() return self:handle_focus_gained() end,
        -- ['FocusLost'] = function() return self:handle_focus_lost() end,
        ['Key'] = function() return self:handle_key_events(event) end,
        ['Mouse'] = function() return self:handle_mouse_events(event) end,
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

    -- Key mapping table for parameter adjustments
    local key_actions = {
        ['+'] = function() self.params.noise_intensity = self.params.noise_intensity + 0.1 end,
        ['-'] = function() self.params.noise_intensity = self.params.noise_intensity - 0.1 end,
        ['Up'] = function()
            self.params.min_brightness = self.params.min_brightness + 0.1
            self.params.max_brightness = self.params.max_brightness + 0.1
        end,
        ['Down'] = function()
            self.params.min_brightness = self.params.min_brightness - 0.1
            self.params.max_brightness = self.params.max_brightness - 0.1
        end,
        ['Right'] = function()
            self.params.min_brightness = self.params.min_brightness - 0.1
            self.params.max_brightness = self.params.max_brightness + 0.1
        end,
        ['Left'] = function()
            self.params.min_brightness = self.params.min_brightness + 0.1
            self.params.max_brightness = self.params.max_brightness - 0.1
            if self.params.min_brightness > self.params.max_brightness then
                self.params.min_brightness, self.params.max_brightness =
                    self.params.max_brightness, self.params.min_brightness
            end
        end,
        ['r'] = function()
            self.params = {
                noise_intensity = 1,
                min_brightness = 0.0,
                max_brightness = 1.0,
            }
        end
    }

    local key = key_event.code
    if key_actions[key] then key_actions[key]() end

    should_update.params = true
    return should_update
end

function Simulation:handle_mouse_events(event)
    local should_update = {
        simulation = false,
        particles = false,
        params = false,
        config = false,
    }

    local mouse_actions = {
        ['ScrollUp'] = function()
            self.config.texture_index = (self.config.texture_index + 1) % #self.textures
        end,
        ['ScrollDown'] = function()
            self.config.texture_index = (self.config.texture_index - 1 + #self.textures) % #self.textures
        end,
    }

    if mouse_actions[event.kind] then mouse_actions[event.kind]() end

    should_update.config = true
    return should_update
end

function Simulation:handle_resize_events(event)
    local noise_intensity = self.params.noise_intensity
    local min_brightness = self.params.min_brightness
    local max_brightness = self.params.max_brightness

    local should_update = {
        simulation = false,
        particles = false,
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
                local particle_brightness = (min_brightness + math.random() *
                    (max_brightness - min_brightness)) * noise_intensity

                result[i][j] = particle_brightness
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
Max brightness: %.2f
Min brightness: %.2f
]],
        self.params.noise_intensity,
        self.params.max_brightness,
        self.params.min_brightness
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
