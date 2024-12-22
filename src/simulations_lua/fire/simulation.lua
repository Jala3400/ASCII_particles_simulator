Simulation = {}
Simulation.__index = Simulation

function Simulation.setup()
    local self = setmetatable({}, Simulation)
    self.particles = { {} }
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

-- fn handle_key(key: KeyEvent, app: &mut App) {
--     use KeyCode::*;
--     match key.code {
--         Char('+') => {
--             app.fire_data.noise_intensity += 0.01;
--         }
--         Char('-') => {
--             app.fire_data.noise_intensity -= 0.01;
--         }
--         Up => {
--             app.fire_data.below_intensity += 0.01;
--         }
--         Down => {
--             app.fire_data.below_intensity -= 0.01;
--         }
--         Right => {
--             app.fire_data.past_intensity += 0.01;
--         }
--         Left => {
--             app.fire_data.past_intensity -= 0.01;
--         }
--         Char('.') => {
--             app.fire_data.fire_intensity += 0.01;
--         }
--         Char(',') => {
--             app.fire_data.fire_intensity -= 0.01;
--         }
--         Char('r') => {
--             let new_fire_data = FireData::new();
--             app.fire_data = new_fire_data;
--         }
--         _ => {
--             app.show_info = false;
--         }
--     }
-- }
