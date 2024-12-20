Particles = { {} }
Params = {
    noise_intensity = 1,
    min_brightness = 0.0,
    max_brightness = 1.0,
}

function Simulate(particles, params)
    -- Default parameters if not provided
    params = params or {
        noise_intensity = 1,
        min_brightness = 0.0,
        max_brightness = 1.0,
    }

    local noise_intensity = 1
    local min_brightness = 0.0
    local max_brightness = 1.0

    local result = {}

    -- Initialize result array with same dimensions
    for i = 1, #particles do
        result[i] = {}
        for j = 1, #particles[1] do
            local particle_brightness = (min_brightness + math.random() *
                (max_brightness - min_brightness)) * noise_intensity

            result[i][j] = particle_brightness
        end
    end

    return result
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
