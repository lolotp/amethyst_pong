use amethyst::{
    derive::SystemDesc,
    core::{
        transform::Transform,
        geometry::Plane,
        math::{Point2, Vector2},
    },
    ecs::{Join, System, Read, ReadStorage, WriteStorage, SystemData},
    renderer::camera::Camera,
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
        ReadStorage<'s, Camera>,
    );

    fn run(&mut self, (pieces, mut locals, input, cameras): Self::SystemData) {
        if let Some(mouse_position) = input.mouse_position() {
            for (camera, camera_transform) in (&cameras, &mut locals).join() {
                let ray = camera.screen_ray(
                    Point2::new(mouse_position.0, mouse_position.1),
                    Vector2::new(
                        // not sure why this is the screen dimesions on my laptop
                        // supposed to be 800,600
                        2000.0, 
                        1500.0,
                    ),
                    camera_transform,
                );
                let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                let mouse_world_position = ray.at_distance(distance);
                println!("mouse_world is {}, {}", mouse_world_position.x, mouse_world_position.y);
                println!("mouse is {}, {}", mouse_position.0, mouse_position.1);
            }
        }
    }
}