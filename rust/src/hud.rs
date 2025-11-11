use godot::classes::{Button, CanvasLayer, ICanvasLayer, Label, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=CanvasLayer)]
pub(crate) struct Hud {
    base: Base<CanvasLayer>,
}

#[godot_api]
impl Hud {
    #[signal]
    pub(crate) fn start_game();

    pub(crate) fn show_message(&mut self, message: &str) {
        let mut label = self.base().get_node_as::<Label>("Message");
        label.set_text(message);
        label.show();

        let mut message_timer = self.base().get_node_as::<Timer>("MessageTimer");
        message_timer.start();
    }

    pub(crate) fn show_game_over(&mut self) {
        self.show_message("Game Over");

        let message_timer = self.base().get_node_as::<Timer>("MessageTimer");
        let mut label = self.base().get_node_as::<Label>("Message");
        let mut scene_tree = self.base_mut().get_tree().unwrap();

        let mut start_button = self.base().get_node_as::<Button>("StartButton");
        godot::task::spawn(async move {
            message_timer.signals().timeout().to_future().await;
            label.set_text("Dodge the Creeps!");
            label.show();

            let new_timer = scene_tree.create_timer(1.0).unwrap();
            new_timer.signals().timeout().to_future().await;

            start_button.show();
        });
    }

    pub(crate) fn update_score(&mut self, score: i64) {
        let mut score_label = self.base().get_node_as::<Label>("ScoreLabel");
        score_label.set_text(&format!("Score: {}", score));
    }
}

#[godot_api]
impl ICanvasLayer for Hud {
    fn ready(&mut self) {
        let message_timer = self.base().get_node_as::<Timer>("MessageTimer");
        message_timer
            .signals()
            .timeout()
            .connect_other(self, |this| {
                this.base_mut().get_node_as::<Label>("Message").hide();
            });

        let start_button = self.base().get_node_as::<Button>("StartButton");
        start_button
            .signals()
            .pressed()
            .connect_other(self, |this| {
                this.base_mut().get_node_as::<Button>("StartButton").hide();
                this.signals().start_game().emit();
            });
    }
}
