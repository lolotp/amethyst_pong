//created by ffreakk

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::math::Vector3,
    core::transform::Transform,
    core::timing::Time,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        camera::Camera,
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
        Texture,
    },
};

#[derive(Default)]
pub struct Pong {
    ball_spawn_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

pub const ARENA_WIDTH: f32 = 800.0;
pub const ARENA_HEIGHT: f32 = 600.0;
pub const BALL_VELOCITY_X: f32 = 0.0;
pub const BALL_VELOCITY_Y: f32 = 0.0;
pub const BALL_RADIUS: f32 = 2.0;

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialise_board(world: &mut World) {
    let sprite_sheet_handle = load_sprite_sheet(world, "board_spritesheet");
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
    local_transform.set_scale(Vector3::new(0.6, 0.6, 1.0));

    // Assign the sprite for the ball. The ball is the second sprite in the sheet.
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    world
        .create_entity()
        .with(sprite_render)
        .with(local_transform)
        .build();
}

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

/// Initialises one ball in the middle-ish of the arena.
fn initialise_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
    local_transform.set_scale(Vector3::new(0.25, 0.25, 1.0));

    // Assign the sprite for the ball. The ball is the second sprite in the sheet.
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .build();
}

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Wait one second before spawning the ball.
        self.ball_spawn_timer.replace(0.0);

        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.
        self.sprite_sheet_handle.replace(load_sprite_sheet(world, "pieces_spritesheet"));

        initialise_camera(world);
        initialise_board(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(mut timer) = self.ball_spawn_timer.take() {
            // If the timer isn't expired yet, subtract the time that passed since the last update.
            {
                let time = data.world.fetch::<Time>();
                timer -= time.delta_seconds();
            }
            if timer <= 0.0 {
                // When timer expire, spawn the ball
                initialise_ball(data.world, self.sprite_sheet_handle.clone().unwrap());
            } else {
                // If timer is not expired yet, put it back onto the state.
                self.ball_spawn_timer.replace(timer);
            }
        }
        Trans::None
    }
}

fn load_sprite_sheet(world: &mut World, spritsheet_name: &str) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("texture/{}.png", spritsheet_name),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("texture/{}.ron", spritsheet_name), // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}