use godot::classes::{AnimatedSprite2D, IRigidBody2D, RigidBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=RigidBody2D)]
pub(crate) struct Mob {
    base: Base<RigidBody2D>,
}

#[godot_api]
impl IRigidBody2D for Mob {

    fn ready(&mut self) {
        let mut animated_sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        let mob_types = animated_sprite.get_sprite_frames().unwrap().get_animation_names();
        let index = godot::global::randi() as usize % mob_types.len();
        let animation = mob_types[index].arg();
        animated_sprite.set_animation(animation);
        animated_sprite.play();

        let screen_notifier = self
            .base()
            .get_node_as::<godot::classes::VisibleOnScreenNotifier2D>("VisibleOnScreenNotifier2D");

        screen_notifier.signals().screen_exited().connect_other(self,  |this| this.base_mut().queue_free() );
    }
}
