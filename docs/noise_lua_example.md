# Ejemplo ruido

La simulación viene de la simulación de [ruido](../simulations_lua/noise/simulation.lua)

Vamos a explicarla paso por paso:

# Índice

- [Ejemplo ruido](#ejemplo-ruido)
- [Índice](#índice)
- [Setup](#setup)
- [Simulate](#simulate)
- [Handle events](#handle-events)
  - [Handle key events](#handle-key-events)
  - [Handle mouse events](#handle-mouse-events)
  - [Handle resize events](#handle-resize-events)
- [Getters y setters](#getters-y-setters)

# Setup

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
        noise_intensity = 1,
        min_brightness = 0.0,
        max_brightness = 1.0,
    }
    return self
end
```

Las dos primeras líneas son necesarias para crear la clase `Simulation`.

El método `setup` inicializa la simulación con los siguientes atributos:

-   `particles` es una tabla de tablas de números que representan la luminosidad de cada partícula.
    Estas son las partículas que se van a mostrar en la simulación. Son solo valores numéricos, la conversión a caracteres se hace en rust.
    Los valores deben ir entre 0 y 1, pero si están fuera de ese rango se ajustarán automáticamente.

-   `textures` es una tabla de tablas de caracteres que representan las partículas.
    Son los posibles caracteres que se pueden usar para representar las partículas. Se pueden añadir más caracteres si se quiere.
    Cada tabla representa una textura. pulsando Enter se cambia a la siguiente textura.
    Deben estar ordenados de menor a mayor luminosidad.

-   `config` es una tabla de configuración de la simulación. Guarda el tiempo entre cada frame y el índice de la textura actual.

-   `params` es una tabla de parámetros de la simulación. No es necesario, pero ayuda a tener localizados los que se muestran por pantalla.

Técnicamente no es necesario tener estos atributos, ya que la aplicación solo se comunicará con la simulación mediante los métodos.

# Simulate

```lua

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
```

Este método se llama antes de actualizar el frame. En este caso, se genera un ruido aleatorio en cada partícula.

Podemos ver que se crea la estructura `should_update` que indica qué atributos deben actualizar. En este caso, solo se marca `particles`.
Luego procesamos las partículas y sustituimos las antiguas por las nuevas.

Siempre se debe devolver `should_update`.

# Handle events

```lua
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
```

Se llama a esta función cada vez que se registra un evento en la aplicación.

Todos los eventos tienen el campo `type`, que indica qué tipo de evento es. En este caso, solo se gestionan los eventos de teclado y de redimensionado.

Siempre se debe gestionar al campo de redimensionado, ya que, si no se hace, la simulación no se verá correctamente.

En este caso la función redirecciona a otras funciones. Para ver todos los detalles de los eventos, se puede mirar en [básicos](basics.md#argumentos).

## Handle key events

```lua

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
```

Este método se llama cuando una tecla ha sido pulsada.

Para seleccionar de forma eficiente la acción a realizar, se usa una tabla de acciones. Cada acción se asocia a una tecla.

Solo se cambian los parámetros, por lo que solo se marca `params` en `should_update`.

## Handle mouse events

```lua
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
```

Este método se llama cuando se ha registrado un evento de ratón. En este caso se puede cambiar la textura actual con la rueda del ratón, por lo que hay que actualizar la configuración.

## Handle resize events

```lua
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
```

Este método se llama cuando se ha redimensionado la consola. En este caso se mantienen las partículas que estaban ya en la simulación y se añaden las nuevas aleatoriamente.

También hay que actualizar las partículas.

# Getters y setters

```lua
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
```

Estos métodos son necesarios para que la aplicación pueda cambiar los valores de la simulación.

Todos siguen el mismo formato menos `get_params`, que devuelve un string con el texto que se quiera.