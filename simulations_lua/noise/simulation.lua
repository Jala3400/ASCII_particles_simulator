Simulation = {}
Simulation.__index = Simulation

function Simulation.setup(particles)
    local self = setmetatable({}, Simulation)
    self.particles = particles
    self.textures = { { ' ', '·', '+', '#' }, { ' ', '.', 'o', '@' } }
    self.config = {
        color_enabled = false,
        mill_per_frame = 250,
    }
    self.params = {
        noise_intensity = 1,
        min_brightness = 0.0,
        max_brightness = 1.0,
        texture_index = 1,
    }
    return self
end

function Simulation:simulate()
    local noise_intensity = self.params.noise_intensity
    local min_brightness = self.params.min_brightness
    local max_brightness = self.params.max_brightness

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
            local particle_brightness = (min_brightness + math.random() *
                (max_brightness - min_brightness)) * noise_intensity

            result[i][j] = particle_brightness
        end
    end

    self.particles = result

    should_update.particles = true
    return should_update
end

function Simulation:handle_key_events(key_event)
    local should_update = {
        particles = false,
        simulation = false,
        params = false,
        config = false,
    }

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

function Simulation:set_particles(particles)
    self.particles = particles or self.particles
end

function Simulation:get_particles()
    return self.particles
end

function Simulation:set_params(params)
    self.params = params or self.params
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
    self.params.texture_index = index
end

function Simulation:get_texture_index()
    return self.params.texture_index
end

function Simulation:set_config(config)
    self.config = config or self.config
end

function Simulation:get_config()
    return self.config
end
