use gdnative::prelude::*;
use gdnative::api::*;

#[derive(NativeClass)]
#[inherit(StaticBody2D)]
pub struct QuestionBlock {
    animation_player: Option<TRef<'static, AnimationPlayer>>,
    blockstate: BlockState
}

enum BlockState {
    Full = 0,
    Empty
}

#[methods]
impl QuestionBlock {
    fn new(_owner: &StaticBody2D) -> Self {
        return Self {
            animation_player: None,
            blockstate: BlockState::Full
        };
    }

    #[export()]
    fn _ready(&mut self, owner: &StaticBody2D) {
        self.animation_player = unsafe { owner.get_node_as::<AnimationPlayer>("AnimationPlayer") };
    }

    #[export()]
    fn _on_hitbox_body_entered(&mut self, _owner: &StaticBody2D, body: Ref<Node>) {
        let player = unsafe { body.assume_safe() };
        if !player.is_in_group("Player") {
            return;
        }

        self.animation_player.unwrap().play("Hit", -1.0, 1.0, false);
    }

    #[export()]
    fn _on_animation_player_animation_finished(&mut self, _owner: &StaticBody2D, name: String) {
        if name == "Hit" {
            self.animation_player.unwrap().play("Empty", -1.0, 1.0, false);
            self.blockstate = BlockState::Empty;
        }
    }
}