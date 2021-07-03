use amethyst::{
    derive::SystemDesc,
    core::transform::Transform,
    ecs::{System, ReadStorage, WriteStorage, SystemData},
};

use crate::chinese_chess::Piece;

#[derive(SystemDesc)]
pub struct MouseRaycastSystem;

impl<'s> System<'s> for MouseRaycastSystem {
    type SystemData = (
        ReadStorage<'s, Piece>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (pieces, mut locals): Self::SystemData) {
    }
}