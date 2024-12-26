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
