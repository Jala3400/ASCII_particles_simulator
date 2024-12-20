function Simulate(particles, params)
    -- Default parameters if not provided
    params = params or {
        noise_intensity = 0.07,
        fire_intensity = 1.0,
        past_intensity = 0.25,
        below_intensity = 0.70
    }

    local noise_intensity = params.noise_intensity
    local fire_intensity = params.fire_intensity
    local past_intensity = params.past_intensity
    local below_intensity = params.below_intensity

    local result = {}

    -- Initialize result array with same dimensions
    for i = 1, #particles do
        result[i] = {}
        for j = 1, #particles[1] do
            local past_brightness = particles[i][j] * past_intensity

            local below_brightness = 1.0
            if i < #particles then
                below_brightness = particles[i + 1][j]
            end
            below_brightness = below_brightness * below_intensity

            local noise_brightness = math.random() * 2.1 - 1.1 * noise_intensity

            local particle_brightness = (past_brightness + below_brightness + noise_brightness)
                * fire_intensity

            result[i][j] = particle_brightness
        end
    end

    return result
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

