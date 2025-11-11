use godot::prelude::*;

#[path = "main_node.rs"]
mod main;
mod mob;
mod player;
mod hud;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}