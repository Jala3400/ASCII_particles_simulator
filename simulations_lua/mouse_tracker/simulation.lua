Simulation = {}
Simulation.__index = Simulation

function Simulation.setup(particles)
    local self = setmetatable({}, Simulation)
    self.particles = particles
    self.textures = { { " ", "-", "|", "/", "\\", "+" } }
    self.config = {
        millis_per_frame = 0,
        texture_index = 0, -- starting at 0, because config is handled in rust
    }
    self.params = {
        cursor_x = 1,
        cursor_y = 1,
    }

    -- Initialize characters enum

    local len = #self.textures[self.config.texture_index + 1]
    self.characters = {
        SPACE = 0 / len,
        DASH = 1 / len,
        VERTICAL_BAR = 2 / len,
        SLASH = 3 / len,
        BACKSLASH = 4 / len,
        ADD = 5 / len,
    }
    return self
end

function Simulation:simulate()
    local should_update = {
        simulation = false,
        particles = true,
        params = false,
        config = false,
    }

    local result = {}

    local chars = self.characters
    local x = self.params.cursor_x
    local y = self.params.cursor_y

    -- Initialize result array with same dimensions
    for i = 1, #self.particles do
        result[i] = {}
        for j = 1, #self.particles[1] do
            if x == j and y == i then
                result[i][j] = chars.ADD
            elseif x == j then
                result[i][j] = chars.VERTICAL_BAR
            elseif y == i then
                result[i][j] = chars.DASH
            elseif x - j == y - i then
                result[i][j] = chars.BACKSLASH
            elseif x - j == i - y then
                result[i][j] = chars.SLASH
            else
                result[i][j] = chars.SPACE
            end
        end
    end

    self.particles = result
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
        ["Key"] = function()
            return self:handle_key_events(event)
        end,
        ["Mouse"] = function()
            return self:handle_mouse_events(event)
        end,
        -- ['Paste'] = function() return self:handle_paste_events(event) end,
        ["Resize"] = function()
            return self:handle_resize_events(event)
        end,
    }

    if events[event.type] then
        should_update = events[event.type]()
    end

    should_update.params = true
    return should_update
end

function Simulation:handle_key_events(key_event)
    local should_update = {
        simulation = false,
        particles = false,
        params = false,
        config = false,
    }

    if key_event.kind == "Release" then
        return should_update
    end

    -- Key mapping table for parameter adjustments
    local key_actions = {
        ["Up"] = function()
            self.params.cursor_y = self.params.cursor_y - 1
        end,
        ["Down"] = function()
            self.params.cursor_y = self.params.cursor_y + 1
        end,
        ["Right"] = function()
            self.params.cursor_x = self.params.cursor_x + 1
        end,
        ["Left"] = function()
            self.params.cursor_x = self.params.cursor_x - 1
        end,
        ["W"] = function()
            self.params.cursor_y = self.params.cursor_y - 1
        end,
        ["S"] = function()
            self.params.cursor_y = self.params.cursor_y + 1
        end,
        ["D"] = function()
            self.params.cursor_x = self.params.cursor_x + 1
        end,
        ["A"] = function()
            self.params.cursor_x = self.params.cursor_x - 1
        end,
        ["w"] = function()
            self.params.cursor_y = self.params.cursor_y - 1
        end,
        ["s"] = function()
            self.params.cursor_y = self.params.cursor_y + 1
        end,
        ["d"] = function()
            self.params.cursor_x = self.params.cursor_x + 1
        end,
        ["a"] = function()
            self.params.cursor_x = self.params.cursor_x - 1
        end,
    }

    local key = key_event.code
    if key_actions[key] then
        key_actions[key]()
    end

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

    self.params.cursor_x = event.x + 1
    self.params.cursor_y = event.y + 1

    return should_update
end

function Simulation:handle_resize_events(event)
    local should_update = {
        simulation = false,
        particles = false,
        params = false,
        config = false,
    }

    self.params.cursor_x = 1
    self.params.cursor_y = 1

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
X: %.2f
Y: %.2f
]],
        self.params.cursor_x,
        self.params.cursor_y
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
