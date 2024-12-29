# Básicos

La aplicación está hecha en rust, y las simulaciones en lua.

Las simulaciones que se ejecutan están en `simulations_lua`, constan de una carpeta con un archivo llamado `simulation.lua`.

Las simulaciones son clases, y rust ejecuta sus métodos cuando corresponde. Estos métodos devuelven una estructura que indica qué parámetros se deben actualizar.

# Índice

- [Básicos](#básicos)
- [Índice](#índice)
- [Crear una simulación](#crear-una-simulación)
  - [Resumen métodos](#resumen-métodos)
    - [Argumentos](#argumentos)
  - [Atributos](#atributos)
- [Ciclo de ejecución](#ciclo-de-ejecución)
  - [Should update](#should-update)

# Crear una simulación

Para crear una simulación, primero se debe crear una carpeta con un archivo llamado `simulation.lua`.

## Resumen métodos

Este archivo contendrá una clase llamada `Simulation` que se comunica con la aplicación mediante los siguientes métodos:

-   `setup(particles)`: Inicializa la simulación.
-   `simulate()`: Se llama antes de actualizar el frame.
-   `handle_events(event)`: Gestiona los eventos de la aplicación (pulsaciones de teclas, resizing...).
-   `set_particles(particles)`: Establece las partículas de la simulación.
-   `get_particles()`: Devuelve las partículas de la simulación.
-   `get_params()`: Devuelve el string que se muestra en pantalla al pulsar la `i`.
-   `set_textures(textures)`: Establece los caracteres que se usan para representar las partículas.
-   `get_textures()`: Devuelve los caracteres que se usan para representar las partículas.
-   `set_texture_index()`: Establece el índice de las texturas (empezando en 0, ya que se usa en rust, no en lua)
-   `get_texture_index()`: Devuelve el índice de las texturas

### Argumentos

Los argumentos que se les pasan a estos métodos son:

-   `particles` es una lista de listas de números que representan la luminosidad de cada partícula. Los valores deben estar entre 0 y 1. La aplicación permite tratar con valores mayores, pero a la hora de representarlos representa los mayores a 1 como 1 y los menores a 0 como 0.
-   `event` es una tabla con diferentes campos dependiendo del evento que sea. Es importante tratar algunos como el `Resize`.

<details>
<summary>Eventos</summary>

Todos tienen el campo `type`. Estos tipos pueden ser:

-   `FocusGained`: Indica que se ha puesto el focus en la consola.
-   `FocusLost`: Indica que la aplicación activa ya no es la consola.
-   `Key`: Ha habido algún evento con las teclas. Esta tabla también tiene los campos
    -   `code`: Texto con la tecla que se ha pulsado. Si se ha pulsado un carácter aparece ese carácter si no se pueden ver los posibles códigos al final de este archivo.
    -   `modifiers`: Una tabla con las teclas modificadoras que están pulsadas (ver al final del archivo.)
    -   `kind`: Press / Repeat / Release
-   `Mouse`: El mouse ha hecho algo:
    -   `x`: La columna en la que se ha disparado el evento
    -   `y`: La fila en la que se ha disparado el evento
    -   `kind`: Down / Up / Drag / Moved / ScrollUp / ScrollDown / ScrollRight / ScrollLeft
    -   `button`: Left / Right / Middle / None. Solo presente en Down, Up y Moved.
    -   `modifiers`: Una tabla con las teclas modificadoras que están pulsadas (ver al final del archivo.)
-   `Paste`: Se ha pegado texto
    -   `text`: El texto que se ha pegado
-   `Resize`: Se ha cambiado el tamaño de la ventana
    -   `x`: El nuevo número de columnas
    -   `y `: El nuevo número de filas
    -   **Es muy recomendable tratar este evento, aunque se pongan todas las partículas a 0, porque al cambiar el tamaño de la matriz con las partículas permite tratarlas en el siguiente frame.** Por ejemplo, si no se trata al hacer la ventana más grande, se quedará el recuadro de tamaño original en la esquina superior izquierda, dejando el resto de la pantalla en negro.

<details>
<summary>Teclas modificadoras:</summary>

Shift = bool
Ctrl = bool
Alt = bool
Super = bool
Hyper = bool
Meta = bool

</details>

<details>
<summary>Otras teclas que no son caracteres son:</summary>

Backspace / Enter / Left / Right / Up / Down / Home / End / PageUp / PageDown / Tab / BackTab / Delete / Insert / F(u8) / Null / Esc / CapsLock / ScrollLock / NumLock / PrintScreen / Pause / Menu / KeypadBegin / Media(MediaKeyCode) / Modifier(ModifierKeyCode)

MediaKeyCode = Play / Pause / PlayPause / Reverse / Stop / FastForward / Rewind / TrackNext / TrackPrevious / Record / LowerVolume / RaiseVolume / MuteVolume /

ModifierKeyCode = LeftShift / LeftControl / LeftAlt / LeftSuper / LeftHyper / LeftMeta / RightShift / RightControl / RightAlt / RightSuper / RightHyper / RightMeta / IsoLevel3Shift / IsoLevel5Shift

</details>

</details>
<br>

-   `textures`: Una lista de listas con los caracteres que se van a usar para representar la luminosidad. `set_textures(textures)` aún no se usa, pero puede que en un futuro sí.

## Atributos

Técnicamente no son obligatorios, pero es recomendable tenerlos para para simplificar la interacción con la aplicación mediante los getters y setters.

-   `particles`: Lista de listas de números que representan la luminosidad de cada partícula.
-   `textures`: Lista de listas con los caracteres que se van a usar para representar la luminosidad.
-   `config`: Tabla con la configuración de la simulación. Tiene los siguientes campos:
    -   `millis_per_frame`: Número de milisegundos que se espera entre frame y frame.
    -   `texture_index`: Índice de las texturas que se están usando (empezando en 0, porque se trata en rust).
-   `params`: Parámetros internos de la aplicación. Este es completamente opcional, pero es útil para guardar información que se va a mostrar en pantalla al pulsar la `i`.

# Ciclo de ejecución

Al comenzar la aplicación, se ejecuta el método `setup` de la simulación. Tras esto, llamarán a todos los getters para dibujar el primer frame.

Después se entra en el bucle principal, que espera el tiempo indicado en `config.millis_per_frame` y llama a `simulate`.

En caso de que se hayan producido eventos, se llamará a `handle_events` con el evento correspondiente.

Estos métodos devuelven una estructura llamada `should_update` que indica qué parámetros se deben actualizar.

## Should update

```lua
    local should_update = {
        simulation = false,
        particles = false,
        params = false,
        config = false,
    }
```

Cada campo indica si se debe actualizar ese parámetro. Si se pone a `true`, se llamará al getter correspondiente inmediatamente.

-   `simulation`: Llama a `simulate` **No se debe activar dentro del método simulate.**
-   `particles`: Llama a `get_particles`.
-   `params`: Llama a `get_params`.
-   `config`: Llama a `get_config`.