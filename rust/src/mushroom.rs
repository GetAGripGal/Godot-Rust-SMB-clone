use gdnative::prelude::*;

pub const SPEED: f32 = 25.0;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Mushroom {
    velocity: Vector2,
    direction: i32
}

#[methods]
impl Mushroom {
    pub fn new(_owner: &KinematicBody2D) -> Self {
        return Self {
            velocity: Vector2::new(0.0, 0.0),
            direction: 1,
        };
    }

    #[export()]
    pub fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f32) {
        if self.velocity.x == 0.0 {
            self.direction = -self.direction;
        }

        self.velocity = Vector2::new(SPEED * (self.direction as f32), 0.0);
        self.velocity = self.move_mushroom(owner);
    }

    pub fn move_mushroom(&mut self, owner: &KinematicBody2D) -> Vector2 {
        return owner.move_and_slide(self.velocity, Vector2::new(0.0, -1.0), false, 4, 0.785398, true);
    }

    #[export()]
    pub fn _on_hitbox_body_entered(&mut self, owner: &KinematicBody2D, body: Ref<Node>) {
        let player = unsafe { body.assume_safe() };
        if !player.is_in_group("Player") {
            return;
        }

        unsafe { player.call("power_up", &[(1).to_variant()]) };
        owner.queue_free();
    }
}