use amethyst::{
    derive::SystemDesc,
    core::{
        transform::Transform,
        geometry::Plane,
        math::{Point2, Vector2},
    },
    ecs::{Join, System, Read, ReadStorage, WriteStorage, ReadExpect, SystemData},
    renderer::{
        camera::Camera,
        resources::Tint,
        palette::Srgba,
    },
    window::ScreenDimensions,
};
use amethyst::input::{InputHandler, StringBindings};

use crate::chinese_chess::Piece;

#[derive(SystemDesc)]
pub struct MouseRaycastSystem;

impl<'s> System<'s> for MouseRaycastSystem {
    type SystemData = (
        ReadStorage<'s, Piece>,
        WriteStorage<'s, Tint>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Camera>,
    );

    fn run(&mut self, (pieces, mut tints, locals, input, screen_dimensions, cameras): Self::SystemData) {
        if let Some(mouse_position) = input.mouse_position() {
            for (camera, camera_transform) in (&cameras, &locals).join() {
                let ray = camera.screen_ray(
                    Point2::new(mouse_position.0, mouse_position.1),
                    Vector2::new(
                        screen_dimensions.width(), 
                        screen_dimensions.height(),
                    ),
                    camera_transform,
                );
                let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                let mouse_world_position = ray.at_distance(distance);
                //println!("mouse_world is {}, {}", mouse_world_position.x, mouse_world_position.y);
                //println!("mouse is {}, {}", mouse_position.0, mouse_position.1);

                for (piece, piece_transform, tint) in (&pieces, &locals, &mut tints).join() {
                    if (f32::abs(piece_transform.translation().x - mouse_world_position.x) < Piece::HITBOX) &&
                     (f32::abs(piece_transform.translation().y - mouse_world_position.y) < Piece::HITBOX) {
                        //println!("I'm in piece {:?}", piece.piece_type);
                        *tint = Tint(Srgba::new(1.3,1.3,1.3,1.0));
                    } else {
                        *tint = Tint(Srgba::new(1.0,1.0,1.0,1.0));
                    }
                }
            }
        }
    }
}