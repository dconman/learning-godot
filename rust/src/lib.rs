use godot::prelude::*;

#[path = "main_node.rs"]
mod main;
mod mob;
mod player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}