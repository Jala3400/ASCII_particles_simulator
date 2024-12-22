Simulation = {}
Simulation.__index = Simulation

function Simulation.setup()
    local self = setmetatable({}, Simulation)
    self.particles = { {} }
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

    return result
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
    return self.params
end

-- fn handle_noise_key(key: KeyEvent, app: &mut App) {
--     use KeyCode::*;
--     match key.code {
--         Char('+') => {
--             app.noise_data.noise_intensity += 0.1;
--         }
--         Char('-') => {
--             app.noise_data.noise_intensity -= 0.1;
--         }
--         Up => {
--             app.noise_data.min_brightness += 0.1;
--             app.noise_data.max_brightness += 0.1;
--         }
--         Down => {
--             app.noise_data.min_brightness -= 0.1;
--             app.noise_data.max_brightness -= 0.1;
--         }
--         Right => {
--             app.noise_data.min_brightness -= 0.1;
--             app.noise_data.max_brightness += 0.1;
--         }
--         Left => {
--             app.noise_data.min_brightness += 0.1;
--             app.noise_data.max_brightness -= 0.1;
--             if app.noise_data.min_brightness > app.noise_data.max_brightness {
--                 let tmp = app.noise_data.min_brightness;
--                 app.noise_data.min_brightness = app.noise_data.max_brightness;
--                 app.noise_data.max_brightness = tmp;
--             }
--         }
--         Char('r') => {
--             let new_noise_info = NoiseData::new();
--             app.noise_data = new_noise_info;
--         }
--         _ => {
--             app.show_info = false;
--         }
--     }
-- }
