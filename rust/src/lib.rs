use gdnative::prelude::*;

mod player;
mod roomba;
mod bricks;
mod mushroom;
mod question_block;

use player::*;
use roomba::*;
use bricks::*;
use mushroom::*;
use question_block::*;

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Roomba>();
    handle.add_class::<Bricks>();
    handle.add_class::<Mushroom>();
    handle.add_class::<QuestionBlock>();
}

godot_init!(init);