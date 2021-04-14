use gdnative::prelude::*;
use gdnative::api::*;

pub const SPEED: f32 = 100.0;
pub const SPRINT_SPEED: f32 = 150.0;
pub const JUMP_FORCE: f32 = 30.0;

pub const GRAVITY: f32 = 10.0;

#[derive(PartialEq, Eq)]
pub enum PowerUp {
    None = 0,
    Big 
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    velocity: Vector2,

    powerup: PowerUp,

    sprite: Option<TRef<'static, Sprite>>,
    animation_player: Option<TRef<'static, AnimationPlayer>>
}

#[methods]
impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        return Self { 
            velocity: Vector2::new(0.0, 0.0),
            powerup: PowerUp::None,
            sprite: None,
            animation_player: None,
        }
    }   

    #[export()]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        self.animation_player = unsafe { owner.get_node_as::<AnimationPlayer>("AnimationPlayer") };
        self.sprite = unsafe { owner.get_node_as::<Sprite>("Sprite") };
    }

    #[export]
    fn _process(&mut self, owner: &KinematicBody2D, _delta: f32) {
        self.animation_player(owner);
    }

    #[export()]
    fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f32) {
        self.velocity = self.move_player(owner);
    }

    #[export()]
    fn get_powerup(&mut self, _owner: &KinematicBody2D) -> i32 {
        return match self.powerup {
            PowerUp::None => 0,
            PowerUp::Big => 1
        };
    }

    fn move_player(&mut self, owner: &KinematicBody2D) -> Vector2 {
        // Apply gravity
        self.velocity.y += GRAVITY;

        // Get input
        let _input = Input::godot_singleton();
        let x_input = (Input::get_action_strength(&_input, "PlayerRight") - Input::get_action_strength(&_input, "PlayerLeft")) as f32;
        let is_jumping = Input::is_action_just_pressed(&_input, "PlayerJump") && owner.is_on_floor();
        let is_sprinting = Input::is_action_pressed(&_input, "PlayerSprint");
    
        // Apply velocity
        
        if is_jumping {
            self.velocity.y = -GRAVITY * JUMP_FORCE;
        }
    
        self.velocity.x = x_input * (if is_sprinting { SPRINT_SPEED } else { SPEED });
        return owner.move_and_slide(self.velocity, Vector2::new(0.0, -1.0), false, 4, 0.785398, true);
    }

    fn animation_player(&mut self, _owner: &KinematicBody2D) {
        if self.velocity.x == 0.0 {
            self.animation_player.unwrap().play(match self.powerup { 
                PowerUp::None => "Small Idle",
                PowerUp::Big => "Big Idle",
            }, -1.0, 1.0, false);
            return;
        }

        self.animation_player.unwrap().play(match self.powerup { 
            PowerUp::None => "Small Walk",
            PowerUp::Big => "Big Walk",
        }, -1.0, 1.0, false);

        if self.velocity.x < 0.0 {
            self.sprite.unwrap().set_deferred("flip_h", true);
        } else if self.velocity.x > 0.0 {
            self.sprite.unwrap().set_deferred("flip_h", false);
        }
    }

    #[export]
    fn hurt_player(&mut self, owner: &KinematicBody2D) {
        godot_print!("Hurting;");
        
        if self.powerup != PowerUp::None {
            self.powerup = PowerUp::None;
            return;
        }

        self.die(owner);
    }

    #[export()]
    fn power_up(&mut self, _owner: &KinematicBody2D, powerup: i32) {
        self.powerup = match powerup {
            0 => PowerUp::None,
            1 => PowerUp::Big,
            _ => PowerUp::None
        };
    }

    fn die(&mut self, owner: &KinematicBody2D) {
        unsafe { owner.get_tree().unwrap().assume_safe().reload_current_scene().expect("Failed to reload scene") };
    } 

    #[export()]
    fn _on_hitbox_body_entered(&mut self, owner: &KinematicBody2D, body: Ref<Node>) {
        let enemy = unsafe { body.assume_safe() };
        if !enemy.is_in_group("Enemy") {
            return;
        }

        self.hurt_player(owner);
    }
}