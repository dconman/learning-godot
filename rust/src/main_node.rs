use godot::classes::{AudioStreamPlayer, Timer};
use godot::classes::{Marker2D, PathFollow2D};
use godot::prelude::*;

use crate::hud::Hud;
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
        self.base().get_node_as::<AudioStreamPlayer>("Music").stop();
        self.base().get_node_as::<AudioStreamPlayer>("DeathSound").play();
        let mut hud_node = self.base().get_node_as::<Hud>("HUD");
        let mut hud = hud_node.bind_mut();
        hud.show_game_over();
    }

    fn new_game(&mut self) {
        self.score = 0;

        {
            let mut hud_node = self.base().get_node_as::<Hud>("HUD");
            let mut hud = hud_node.bind_mut();
            hud.update_score(self.score);
            hud.show_message("Get Ready!");
        }

        let start_position = self
            .base()
            .get_node_as::<Marker2D>("StartPosition")
            .get_position();
        {
            let mut player = self.base().get_node_as::<Player>("Player");
            player.bind_mut().start(start_position);
        }

        self.base().get_node_as::<AudioStreamPlayer>("Music").play();

        self.base()
            .get_tree()
            .unwrap()
            .call_group("mobs", "queue_free", &[]);

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

    fn on_score_timer_timeout(&mut self) {
        self.score += 1;
        let mut hud_node = self.base().get_node_as::<Hud>("HUD");
        let mut hud = hud_node.bind_mut();
        hud.update_score(self.score);
    }
}

#[godot_api]
impl INode for Main {
    fn ready(&mut self) {
        let base = self.base();
        base.get_node_as::<Player>("Player")
            .signals()
            .hit()
            .connect_other(self, |arg0: &mut Main| Self::game_over(arg0));

        base.get_node_as::<Timer>("ScoreTimer")
            .signals()
            .timeout()
            .connect_other(self, Self::on_score_timer_timeout);

        base.get_node_as::<Timer>("StartTimer")
            .signals()
            .timeout()
            .connect_other(self, |this| this.on_start_timer_timeout());

        base.get_node_as::<Timer>("MobTimer")
            .signals()
            .timeout()
            .connect_other(self, |this| this.on_mob_timer_timeout());

        base.get_node_as::<Hud>("HUD")
            .bind_mut()
            .signals()
            .start_game()
            .connect_other(self, |this| {
                this.new_game();
            });
    }
}
