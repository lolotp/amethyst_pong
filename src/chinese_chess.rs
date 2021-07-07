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
pub struct ChineseChess {
    ball_spawn_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

pub const ARENA_WIDTH: f32 = 800.0;
pub const ARENA_HEIGHT: f32 = 600.0;
pub const CELL_WIDTH: f32 = 100.0;
pub const CELL_HEIGHT: f32 = 102.0;
pub const RIVER_WIDTH: f32 = 104.0;
pub const PIECE_VELOCITY: f32 = 5.0;

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

// (0,0) == bottom left from user's point of view
const NUM_ROWS: i32 = 10;
const NUM_COLUMNS: i32 = 9;
const MID_ROW: f32 = 4.5;
const MID_COLUMN: f32 = 4.0;
const BOARD_SCALE_FACTOR:f32 = 0.60;
fn get_board_coordinates(row: i32, column: i32) -> (f32, f32) {
    let x = ARENA_WIDTH/2.0 - BOARD_SCALE_FACTOR * (MID_COLUMN-column as f32) * CELL_WIDTH;
    let y = ARENA_HEIGHT/2.0 - BOARD_SCALE_FACTOR * ((MID_ROW-row as f32).trunc() * CELL_HEIGHT 
                + (if (row as f32) < MID_ROW {RIVER_WIDTH/2.0} else {-RIVER_WIDTH/2.0}));
    return (x,y)
}

fn initialise_board(world: &mut World) {
    let sprite_sheet_handle = load_sprite_sheet(world, "board_spritesheet");
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
    local_transform.set_scale(Vector3::new(BOARD_SCALE_FACTOR, BOARD_SCALE_FACTOR, 1.0));

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    world
        .create_entity()
        .with(sprite_render)
        .with(local_transform)
        .build();
}

#[derive(Copy, Clone)]
pub enum PieceType {
    Pawn,
    Horse,
    Cannon,
    Elephant,
    Scholar,
    Chariot,
    General,
}

#[derive(Copy, Clone)]
pub enum Side {
    Red, // Tiên
    Black, // Hậu
}

pub struct Piece {
    pub piece_type: PieceType,
    pub side: Side,
    pub position: [i32; 2], //row, column
    pub velocity: [f32; 2],
}

impl Piece {
    fn new(piece_type: PieceType, side: Side, piece_index: i32) -> Piece {
        return Piece {
            piece_type: piece_type,
            side: side,
            position: get_starting_position(piece_type, side, piece_index),
            velocity: [0.0, 0.0],
        };
    }
}

impl Component for Piece {
    type Storage = DenseVecStorage<Self>;
}

fn get_starting_position(piece_type: PieceType, side: Side, _piece_index: i32) -> [i32;2] {
    // TODO(lolotp): take into account _piece_index as well as we
    // start with 5 pawns, 2 cannons, 2 horses, 2 scholars and 2 chariots
    match side {
        Side::Red => match piece_type {
            PieceType::Pawn => [3,0],
            PieceType::Horse => [0,1],
            PieceType::Cannon => [2,1],
            PieceType::Elephant => [0,2],
            PieceType::Scholar => [0,3],
            PieceType::Chariot => [0,0],
            PieceType::General => [0,4],
        },
        Side::Black => match piece_type {
            PieceType::Pawn => [6,0],
            PieceType::Horse => [9,1],
            PieceType::Cannon => [2,1],
            PieceType::Elephant => [7,2],
            PieceType::Scholar => [9,3],
            PieceType::Chariot => [9,0],
            PieceType::General => [9,4],
        },
    }
}

fn get_sprite_index(piece_type: PieceType, side: Side) -> usize {
    match side {
        Side::Red => match piece_type {
            PieceType::General => 0,
            PieceType::Scholar => 1,
            PieceType::Horse => 2,
            PieceType::Elephant => 3,
            PieceType::Chariot => 4,
            PieceType::Cannon => 5,
            PieceType::Pawn => 6,
            
        },
        Side::Black => match piece_type {
            PieceType::General => 7,
            PieceType::Scholar => 8,
            PieceType::Horse => 9,
            PieceType::Elephant => 10,
            PieceType::Chariot => 11,
            PieceType::Cannon => 12,
            PieceType::Pawn => 13,
        },
    }
}

/// Initialises one ball in the middle-ish of the arena.
fn initialise_piece(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let general_piece = Piece::new(PieceType::Chariot, Side::Red, 0);
    // Create the translation.
    let mut local_transform = Transform::default();
    let (x,y) = get_board_coordinates(general_piece.position[0], general_piece.position[1]);
    //println!("piece coordinates are {}, {}", x, y);
    local_transform.set_translation_xyz(x, y, 0.0);
    local_transform.set_scale(Vector3::new(0.15, 0.15, 1.0));

    let sprite_render = SpriteRender::new(sprite_sheet_handle, get_sprite_index(general_piece.piece_type, general_piece.side));

    world
        .create_entity()
        .with(sprite_render)
        .with(general_piece)
        .with(local_transform)
        .build();
}

impl SimpleState for ChineseChess {
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
                initialise_piece(data.world, self.sprite_sheet_handle.clone().unwrap());
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