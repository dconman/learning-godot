use godot::classes::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, Input};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub(crate) struct Player {
    #[export]
    #[init(val = 400.0)]
    speed: f32,
    #[init(val = Vector2::default())]
    screen_size: Vector2,

    base: Base<Area2D>,
}

#[godot_api]
impl Player {
    #[signal]
    pub(crate) fn hit();

    fn on_body_entered(&mut self, _body: Gd<Node2D>) {
        let mut base = self.base_mut();
        base.hide();
        base.emit_signal("hit", &[]);
        base.get_node_as::<CollisionShape2D>("CollisionShape2D")
            .set_deferred("disabled", &Variant::from(true));
    }

    pub(crate) fn start(&mut self, position: Vector2) {
        let mut base = self.base_mut();
        base.get_node_as::<CollisionShape2D>("CollisionShape2D")
            .set_deferred("disabled", &Variant::from(false));
        base.show();
        base.set_position(position);
    }
}

#[godot_api]
impl IArea2D for Player {
    fn ready(&mut self) {
        let viewport = self.base().get_viewport().unwrap();
        let size = viewport.get_visible_rect().size;
        self.screen_size = size;
        self.signals()
            .body_entered()
            .connect_self(Self::on_body_entered);
    }

    fn process(&mut self, _delta: f64) {
        let mut velocity = Vector2::default();

        if Input::singleton().is_action_pressed("move_right") {
            velocity.x += 1.0;
        }
        if Input::singleton().is_action_pressed("move_left") {
            velocity.x -= 1.0;
        }
        if Input::singleton().is_action_pressed("move_down") {
            velocity.y += 1.0;
        }
        if Input::singleton().is_action_pressed("move_up") {
            velocity.y -= 1.0;
        }
        let mut animated_sprite = self
            .base_mut()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * self.get_speed();
            animated_sprite.play();
        } else {
            animated_sprite.stop();
        }

        if velocity.x != 0.0 {
            animated_sprite.set_animation("walk");
            animated_sprite.set_flip_v(false);
            animated_sprite.set_flip_h(velocity.x < 0.0);
        } else if velocity.y != 0.0 {
            animated_sprite.set_animation("up");
            animated_sprite.set_flip_v(velocity.y > 0.0);
        }

        let new_position = (self.base().get_position() + velocity * _delta as f32)
            .clamp(Vector2::ZERO, self.screen_size);

        self.base_mut().set_position(new_position);
    }
}
