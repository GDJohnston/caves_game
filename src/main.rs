mod cell;
mod render;
mod game_scene;
mod main_menu_scene;

use macroquad::prelude::*;
use crate::render::Render;

const MAX_LENGTH:u32 = 7;

struct ColourScheme {
    background_colour: Color,
    tile_colour: Color,
    seperator_colour: Color,
}

const DARK_SCHEME: ColourScheme = ColourScheme{ background_colour: BLACK, tile_colour: DARKGRAY, seperator_colour: LIGHTGRAY };

enum Scene {
    MainMenu,
    Game,
}

#[macroquad::main("Caves")]
async fn main() {
    let mut scene = Scene::MainMenu;

    let mut game_scene = game_scene::GameScene::new();
    let main_menu_scene = main_menu_scene::MainMenu{};
    loop {
        let global_cell = cell::Cell::new(
            (0.0, 0.0),
            (screen_width(), screen_height())
        );

        match scene {
            Scene::MainMenu => {
                    main_menu_scene.logic(&mut scene);
                    main_menu_scene.render(&global_cell);
            }
            Scene::Game => {
                    game_scene.logic();
                    game_scene.render(&global_cell);
            }
        }
        next_frame().await;
    }
}
