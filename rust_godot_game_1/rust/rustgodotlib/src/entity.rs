use hecs::*;
use godot::prelude::*;
use godot::classes::{Node, INode};
use crate::world::HecsWorld;
#[derive(GodotClass)]
#[class(no_init, base=Node)]
pub struct HecsEntity {
    entity: Entity,
    base: Base<Node>
}

impl HecsEntity {
    pub fn from_world(entity:Entity) -> Gd<Self> {
        // Function contains a single statement, the `Gd::from_init_fn()` call.
        
        Gd::from_init_fn(|base| {
            // Accept a base of type Base<Node3D> and directly forward it.
            Self {
                entity,
                base,
            }
        })
    }
}