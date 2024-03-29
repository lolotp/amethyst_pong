mod chinese_chess;
mod systems; 

// created by ffreakk

use crate::chinese_chess::ChineseChess;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use amethyst::input::{InputBundle, StringBindings};
use amethyst::ui::{RenderUi, UiBundle};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?        
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::default())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(systems::MovePiecesSystem, "pieces_system", &[])
        .with(systems::MouseRaycastSystem, "mouse_raycast_system", &["input_system"]);

    let mut game = Application::new(assets_dir, ChineseChess::default(), game_data)?;
    game.run();

    Ok(())
}
