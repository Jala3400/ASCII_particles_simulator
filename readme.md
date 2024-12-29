# ASCII particles simulator

Esta aplicación es un "simulador" de partículas en ASCII. Permite que simulaciones hechas en lua sean representadas en la consola de comandos. Hecho en rust.

Destaca por la capacidad de cambiar de simulación manteniendo las partículas de la anterior.

# Índice

- [ASCII particles simulator](#ascii-particles-simulator)
- [Índice](#índice)
- [Instalación](#instalación)
- [Uso básico](#uso-básico)
- [Añadir/crear simulaciones](#añadircrear-simulaciones)

# Instalación

- Prerrequisitos:
    - [Rust](https://www.rust-lang.org/tools/install)
    - [Git](https://git-scm.com/downloads)

- Pasos:
1. Descarga este repositorio con:
    ```
    git clone https://github.com/Jala3400/ASCII_particles_simulator
    ```
2. Compila el repositorio: En la consola de comandos ejecuta
    ```
    cargo build --release
    ```
3. El ejecutable estará en `./target/release/ascii_particles_simulator.exe`. Para ejecutarlo, escribe en la consola:
    ```
    ./target/release/ascii_particles_simulator.exe
    ```

# Uso básico

Nada más empezar ya hay una simulación corriendo. Las simulaciones se encuentran en la carpeta `simulations_lua`.

Cada simulación decide qué hace cada tecla, pero la aplicación escucha ciertas teclas:

-   q: Salir
-   i: Mostrar información
-   Enter: Cambiar los caracteres mostrados
-   Tab: Cambiar la simulación

# Añadir/crear simulaciones

Para añadir una simulación se debe copiar una carpeta con una simulación en `simulations_lua`.

Para crear una, se puede seguir la [guía](docs/basics.md) o mirar el ejemplo de [ruido](docs/noise_lua_example.md).