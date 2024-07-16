use godot::prelude::*;
pub mod player;
pub mod world;
pub mod entity;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}