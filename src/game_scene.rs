use ::rand::Rng;
use ::rand;

mod cave;
mod inventory;
mod player;
mod point;

use crate::cell;
use crate::render;

use macroquad::prelude::*;

const UP: point::Point = point::Point(0, -1);
const DOWN: point::Point = point::Point(0, 1);
const RIGHT: point::Point = point::Point(1, 0);
const LEFT: point::Point = point::Point(-1, 0);

struct Ui {
    caves_per: f32,
    invent_per: f32,
}

pub struct GameScene {
    cave_rows: u32,
    cave_columns: u32,
    player: player::Player,
}

impl GameScene {
    pub fn new() -> Self {
        let cave_rows = rand::rng().random_range(1..=7);
        let cave_columns = rand::rng().random_range(1..=7);
        let player = player::Player {
            pos: point::Point(0,0),
            cave_columns,
            cave_rows,
        };
        Self {  
            cave_rows,
            cave_columns,
            player,
        }
    }

    fn render_fps() {
        draw_text(format!("FPS: {}", get_fps()).as_str(), 10.0, 20.0, 20., DARKGRAY);
    }
}

impl render::Render for GameScene {
    fn render(self: &Self, cell: &cell::Cell) {
        let ui = Ui{ caves_per: 0.7, invent_per: 0.3 };
        let caves_cell = cell::Cell::new((0.0, 0.0), (cell.x_most * ui.caves_per, cell.y_most));
        let rightpane_cell = cell::Cell::new((cell.x_most * ui.caves_per, 0.0), (cell.x_most, cell.y_most));

        clear_background(BLACK);
        // Seperator
        draw_line(0.7 * screen_width(), 0.0, 0.7 * screen_width(), screen_height(), 2.0, LIGHTGRAY);
        let caves = cave::Cave {rows: self.cave_rows, columns: self.cave_columns};
        caves.render(&caves_cell);
        self.player.render(&caves_cell);
        let inventory = inventory::Inventory { rows: 7, columns: 4};
        inventory.render(&rightpane_cell);
        GameScene::render_fps();
    }
}

impl GameScene {
    pub fn logic(self: &mut Self) {
        if is_key_pressed(KeyCode::W) {
            self.player.move_player(UP);
        } else if is_key_pressed(KeyCode::A) {
            self.player.move_player(LEFT);
        } else if is_key_pressed(KeyCode::S) {
            self.player.move_player(DOWN);
        } else if is_key_pressed(KeyCode::D) {
            self.player.move_player(RIGHT);
        }
    }
}
