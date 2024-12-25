# ASCII particles simulator

Esta aplicación es un "simulador" de partículas en ASCII. Permite que simulaciones hechas en lua sean representadas en la consola de comandos. Hecho en rust.

Destaca por la capacidad de cambiar de simulación manteniendo las partículas de la anterior.

https://github.com/user-attachments/assets/ab796276-0f07-45f6-aede-e27149bd4eb2

## Índice

- [ASCII particles simulator](#ascii-particles-simulator)
  - [Índice](#índice)
  - [Instalación](#instalación)
  - [Uso básico](#uso-básico)
  - [Crear tu simulación](#crear-tu-simulación)
    - [Ejemplo ruido](#ejemplo-ruido)
      - [Setup](#setup)
      - [Simulate](#simulate)
      - [Handle key events](#handle-key-events)
      - [Getters y setters](#getters-y-setters)

## Instalación

1. Descarga rust desde https://www.rust-lang.org/tools/installs:
   Al terminar la instalación escribe el siguiente comando en la consola para comprobar que todo ha salido bien:
    ```
    rustc --version
    ```
2. Descarga este repositorio con:
    ```
    git clone https://github.com/Jala3400/ASCII_particles_simulator
    ```
3. Compila el repositorio: En la consola de comandos ejecuta
    ```
    cargo build --release
    ```
4. El ejecutable estará en `./target/release/ascii_particles_simulator.exe`. Para ejecutarlo, escribe en la consola:
    ```
    ./target/release/ascii_particles_simulator.exe
    ```

## Uso básico

Nada más empezar ya hay una simulación corriendo. Las simulaciones se encuentran en la carpeta `simulations_lua`.

Cada simulación decide qué hace cada tecla, pero la aplicación tiene teclas reservadas:

-   q: Salir
-   i: Mostrar información
-   Enter: Cambiar los caracteres mostrados
-   Tab: Cambiar la simulación

## Crear tu simulación

Para crear una simulación, primero tienes que crear una carpeta con un archivo llamado `simulation.lua`.

Este archivo contendrá una clase llamada `Simulation` con (como mínimo) los siguientes métodos:

-   `setup(particles)`: Inicializa la simulación. `particles` es una tabla de tablas de números que representan la luminosidad de cada partícula.
-   `simulate()`: Devuelve una tabla de tablas de números que representan la luminosidad de cada partícula.
-   `set_particles(particles)`: Cambia las partículas de la simulación.
-   `get_particles()`: Devuelve las partículas de la simulación.
-   `set_params(params)`: Cambia los parámetros de la simulación.
-   `get_params()`: Devuelve los parámetros de la simulación.
-   `set_textures(textures)`: Cambia los caracteres que se usan para representar las partículas.
-   `get_textures()`: Devuelve los caracteres que se usan para representar las partículas.
-   `handle_key_events(key_event)`: Cambia los parámetros de la simulación según la tecla pulsada.

### Ejemplo ruido

Por ejemplo, la siguiente simulación es una simulación de ruido:

```lua
Simulation = {}
Simulation.__index = Simulation

function Simulation.setup(particles)
    local self = setmetatable({}, Simulation)
    self.particles = particles
    self.textures = { { ' ', '·', '+', '#' }, { ' ', '.', 'o', '@' } }
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

function Simulation:handle_key_events(key_event)
    local key = key_event.code
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

    if key_actions[key] then key_actions[key]() end
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
    return {
        { name = "noise_intensity", value = self.params.noise_intensity },
        { name = "min_brightness",  value = self.params.min_brightness },
        { name = "max_brightness",  value = self.params.max_brightness }
    }
end

function Simulation:set_textures(textures)
    self.textures = textures or self.textures
end

function Simulation:get_textures()
    return self.textures
end
```

Vamos a explicarla paso por paso:

#### Setup

```lua
Simulation = {}
Simulation.__index = Simulation

function Simulation.setup(particles)
    local self = setmetatable({}, Simulation)
    self.particles = particles
    self.textures = { { ' ', '·', '+', '#' }, { ' ', '.', 'o', '@' } }
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
    Estas son las partículas que se van a mostrar en la simulación. Son solo valores numéricos, la conversión a caracteres se hace en el backend.
    Los valores deben ir entre 0 y 1, pero si están fuera de ese rango se ajustarán automáticamente.

-   `textures` es una tabla de tablas de caracteres que representan las partículas.
    Son los posibles caracteres que se pueden usar para representar las partículas. Se pueden añadir más caracteres si se quiere.
    Cada tabla representa una textura. pulsando Enter se cambia a la siguiente textura.
    Deben estar ordenados de menor a mayor luminosidad.

-   `params` es una tabla de parámetros de la simulación.
    Son los parámetros que va a tener en cuenta la simulación.

Técnicamente no es necesario tener estos atributos, ya que el backend solo se comunicará con la simulación a través de los métodos.

#### Simulate

```lua
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
```

El método `simulate` devuelve una tabla de tablas (lista de listas) con los números que representan la luminosidad de cada partícula.

Esta simulación, al ser ruido, no tiene en cuenta los valores previos de las partículas y no los guarda. Si se quiere ver un ejemplo teniendo en cuenta los valores previos, se puede ver el [ejemplo de fuego](simulations_lua/fire/simulation.lua).

#### Handle key events

```lua

function Simulation:handle_key_events(key_event)
    local key = key_event.code
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

    if key_actions[key] then key_actions[key]() end
end
```

Este método se llama cuando una tecla ha sido pulsada, salvo que esta sea de las teclas reservadas (q, i, Enter, Tab).

`key_event` tiene el siguiente formato:

```lua
{
    code = "tecla",
    modifiers = {
        shift = true/false,
        ctrl = true/false,
        alt = true/false,
        super = true/false,
        hyper = true/false,
        meta = true/false
    }
    kind = "Press/Repeat/Release"
}
```

Por ahora no se envían eventos de Release.

#### Getters y setters

```lua
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
    return {
        { name = "noise_intensity", value = self.params.noise_intensity },
        { name = "min_brightness",  value = self.params.min_brightness },
        { name = "max_brightness",  value = self.params.max_brightness }
    }
end

function Simulation:set_textures(textures)
    self.textures = textures or self.textures
end

function Simulation:get_textures()
    return self.textures
end
```

Estos métodos son necesarios para que el backend pueda cambiar los valores de la simulación.

Por el momento `get_particles`, `set_params` y `set_textures` no se usan, pero se recomienda tenerlos por si en el futuro se quieren añadir más funcionalidades. (las partículas se obtienen con `simulate`)

-   `set_particles` se usa al cambiar el tamaño de la consola. Por defecto se ponen todas las partículas a 0. Puede que en el futuro se deje en manos del usuario.
-   `get_params` se usa para mostrar la información de la simulación. Con ese formato se puede elegir el orden y el nombre de los parámetros.
-   `get_textures` se usa para obtener los caracteres que se van a usar en la simulación.
