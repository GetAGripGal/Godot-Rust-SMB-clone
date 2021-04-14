use gdnative::prelude::*;
use gdnative::api::*;

pub const GRAVITY: f32 = 10.0;
pub const SPEED: f32 = 50.0;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Roomba {
    velocity: Vector2,

    animation_player: Option<TRef<'static, AnimationPlayer>>,
    direction: i32,

    sprite: Option<TRef<'static, Sprite>>
}

#[methods]
impl Roomba {
    fn new(_owner: &KinematicBody2D) -> Self {
        return Self {
            velocity: Vector2::new(0.0, 0.0),
            animation_player: None,
            direction: 1,
            sprite: None
        };
    }

    #[export()]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        self.animation_player = unsafe { owner.get_node_as::<AnimationPlayer>("AnimationPlayer") };
        self.sprite = unsafe { owner.get_node_as::<Sprite>("Sprite") };
    }

    #[export()]
    fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f32) {
        self.animate_enemy();
        self.velocity = self.move_enemy(owner);
    }

    fn move_enemy(&mut self, owner: &KinematicBody2D) -> Vector2 {
        if self.velocity.x == 0.0 {
            self.direction = -self.direction;
        }

        self.velocity.y += GRAVITY;
        self.velocity.x = SPEED * (self.direction as f32);

        return owner.move_and_slide(self.velocity, Vector2::new(0.0, -1.0), false, 4, 0.785398, true);
    }

    fn animate_enemy(&mut self) {
        if self.velocity.x != 0.0 {
            self.animation_player.unwrap().play("Walk", -1.0, 1.0, false);
        } else {
            self.animation_player.unwrap().play("Idle", -1.0, 1.0, false);
        }

        if self.velocity.x < 0.0 {
            self.sprite.unwrap().set_deferred("flip_h", false);
        } else if self.velocity.x > 0.0 {
            self.sprite.unwrap().set_deferred("flip_h", true);
        }
    }

    #[export()]
    fn _on_hitbox_body_entered(&mut self, owner: &KinematicBody2D, body: Ref<PhysicsBody2D>) {
        let player = unsafe { body.assume_safe() };
        if !player.is_in_group("Player") {
            return;
        }

        owner.queue_free();
    }
}