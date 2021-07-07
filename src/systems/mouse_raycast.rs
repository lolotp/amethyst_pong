use amethyst::{
    derive::SystemDesc,
    core::transform::Transform,
    ecs::{System, Read, ReadStorage, WriteStorage, SystemData},
};
use amethyst::input::{InputHandler, StringBindings};

use crate::chinese_chess::Piece;

#[derive(SystemDesc)]
pub struct MouseRaycastSystem;

impl<'s> System<'s> for MouseRaycastSystem {
    type SystemData = (
        ReadStorage<'s, Piece>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (pieces, mut locals, input): Self::SystemData) {
        if let Some(mouse_position) = input.mouse_position() {
            println!("mouse is {}, {}", mouse_position.0, mouse_position.1);
        }
    }
}