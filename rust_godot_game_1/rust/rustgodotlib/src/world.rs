use hecs::*;
use godot::prelude::*;
use godot::classes::{Node, INode};
use crate::entity::HecsEntity;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct HecsWorld {
    pub world:World,
    base: Base<Node>
}

#[godot_api]
impl HecsWorld {
    #[func]
    pub fn add_component(&mut self, &mut node: Gd<Node>) {
        if (self.base().instance_id() != node.instance_id())
        {
            godot_print!("Add Hecs Component ! {:?}", node);
            let node_path = node.get_path().to_string();
            let entity = self.world.spawn((*node_path));
            let mut entity_node: Gd<Node> = HecsEntity::from_world(entity).upcast();
            
            let entity_name = format!("HecsEntity_{}", entity.id());
            entity_node.set_name(GString::from(entity_name));
            node.add_child(entity_node)
            godot_print!("Hecs Entity : {:?}", node.get_children());
        }
    }

}


#[godot_api]
impl INode for HecsWorld {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Init Hecs World!"); // Prints to the Godot console
        Self {
            world: World::new(),
            base
        }
    }

    fn enter_tree(&mut self) {
        let tree_result = self.base().get_tree();
        let mut tree = tree_result.unwrap();
        tree.connect(StringName::from("node_added"), self.base().callable("add_component"));
    }
}