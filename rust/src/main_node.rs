use godot::classes::Timer;
use godot::classes::{Marker2D, PathFollow2D};
use godot::prelude::*;

use crate::mob::Mob;
use crate::player::Player;

fn randf_range(min: f32, max: f32) -> f32 {
    godot::global::randf_range(min as f64, max as f64) as f32
}

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Main {
    #[export]
    mob_scene: OnEditor<Gd<PackedScene>>,

    score: i64,

    base: Base<Node>,
}

#[godot_api]
impl Main {
    fn game_over(&self) {
        self.base().get_node_as::<Timer>("MobTimer").stop();
        self.base().get_node_as::<Timer>("ScoreTimer").stop();
    }

    fn new_game(&mut self) {
        self.score = 0;

        let start_position = self
            .base()
            .get_node_as::<Marker2D>("StartPosition")
            .get_position();
        let mut player = self.base().get_node_as::<Player>("Player");
        player.bind_mut().start(start_position);

        self.base().get_node_as::<Timer>("StartTimer").start();
    }

    fn on_start_timer_timeout(&mut self) {
        self.base().get_node_as::<Timer>("MobTimer").start();
        self.base().get_node_as::<Timer>("ScoreTimer").start();
    }

    fn on_mob_timer_timeout(&mut self) {
        let mut mob = self.mob_scene.instantiate_as::<Mob>();
        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");
        mob_spawn_location.set_progress_ratio(randf_range(0.0, 1.0));

        let direction = mob_spawn_location.get_rotation()
            + std::f32::consts::FRAC_PI_2
            + randf_range(-std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4);
        let velocity = Vector2::new(randf_range(150.0, 250.0), 0.0).rotated(direction);

        {
            let mut mob_bind = mob.bind_mut();
            let mut mob_base = mob_bind.base_mut();
            mob_base.set_position(mob_spawn_location.get_position());
            mob_base.set_rotation(direction);
            mob_base.set_linear_velocity(velocity);
        }

        self.base_mut().add_child(&mob);
    }
}

#[godot_api]
impl INode for Main {
    fn ready(&mut self) {
        self.base()
            .get_node_as::<Player>("Player")
            .signals()
            .hit()
            .connect_other(self, |this| this.game_over());

        self.base()
            .get_node_as::<Timer>("ScoreTimer")
            .signals()
            .timeout()
            .connect_other(self, |this| this.score += 1);

        self.base()
            .get_node_as::<Timer>("StartTimer")
            .signals()
            .timeout()
            .connect_other(self, |this| this.on_start_timer_timeout());

        self.base()
            .get_node_as::<Timer>("MobTimer")
            .signals()
            .timeout()
            .connect_other(self, |this| this.on_mob_timer_timeout());

        self.new_game();
    }
}
