use gdnative::prelude::*;
use gdnative::api::*;

#[derive(NativeClass)]
#[inherit(StaticBody2D)]
pub struct Bricks {
    animation_player: Option<TRef<'static, AnimationPlayer>>,
    particles: Option<TRef<'static, Particles2D>>,
    sprite: Option<TRef<'static, Sprite>>,
    collision: Option<TRef<'static, CollisionShape2D>>,

    is_broken: bool
}


#[methods]
impl Bricks {
    fn new(_owner: &StaticBody2D) -> Self {
        return Self { 
            animation_player: None,
            particles: None,
            sprite: None,
            collision: None,

            is_broken: false
        };
    }

    #[export()]
    fn _ready(&mut self, owner: &StaticBody2D) {
        self.animation_player = unsafe { owner.get_node_as::<AnimationPlayer>("AnimationPlayer") };
        self.particles = unsafe { owner.get_node_as::<Particles2D>("Particles2D") };
        self.sprite = unsafe { owner.get_node_as::<Sprite>("Sprite") };
        self.collision = unsafe { owner.get_node_as::<CollisionShape2D>("CollisionShape2D") };
    }

    #[export()]
    fn _process(&mut self, owner: &StaticBody2D, _delta: f32) {
        if self.is_broken && !self.particles.unwrap().is_emitting() {
            owner.queue_free();
        }
    }

    #[export()]
    fn on_body_entered_hitbox(&mut self, owner: &StaticBody2D, body: Ref<PhysicsBody2D>) {
        let player = unsafe { body.assume_safe() }; 
        if !player.is_in_group("Player") {
            return;
        }

        if unsafe { player.call("get_powerup", &[]).to_i64() == 0 } {
            self.animation_player.unwrap().play("Hit", -1.0, 1.0, false);   
        } else {
            self.destroy(owner);
        }
    }

    #[export()] 
    fn destroy(&mut self, _owner: &StaticBody2D) {
        self.collision.unwrap().set_deferred("disabled", true);
        self.sprite.unwrap().hide();
        self.particles.unwrap().set_emitting(true);
        self.is_broken = true;
    }
}