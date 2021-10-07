# 🌶 Bevy Spicy Gamebase

This repository serves as a template for your 2D games. It has the following features:

- Integrated Aseprite support (through the [`bevy_spicy_aseprite`](https://github.com/TheNeikos/bevy_spicy_aseprite) crate)
  - Compile time integration of slices/tags
  - Animation support
- Integrated LDTK support (through the [`bevy_spicy_ldtk`](https://github.com/TheNeikos/bevy_spicy_ldtk) crate)
  - Compile time integration of custom enums/entities/level information
  - Bevy adapted loading
- Tight bevy integration, with hot-reloading included!

It is meant as an experiment to see how ergonomic bevy game development can be

## Project Structure

- `main.rs`
  - The entry point of the project
  - The `GameAssets` structure is located here.
    - You can extend it with your own assets (Don't forget to add them to the load tracker just below it)
- `startup.rs`
  - One time setup for the _whole_ project
  - Per-stage startup systems should be registered in the respective stage file
- `utils.rs`
  - Various helper and extension traits
- `stages/`
  - Each stage has its own module
  - `loading.rs`
    - Handles showing the loading screen and waits until all assets in `GameAssets` are done loading
  - `main_menu.rs`
    - The main menu, it handles starting the game/configuration/save games
