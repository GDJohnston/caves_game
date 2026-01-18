mod cave;
mod inventory;
mod point;

use crate::cell;
use crate::render;

use macroquad::prelude::*;

struct Ui {
    caves_per: f32,
    invent_per: f32,
}

pub struct GameScene {
    cave: cave::Cave,
    inventory: inventory::Inventory,
}

impl GameScene {
    pub fn new() -> Self {
        let cave = cave::Cave::new();
        let inventory = inventory::Inventory { rows: 7, columns: 4};
        Self {
            cave,
            inventory,
        }
    }

    fn render_fps() {
        draw_text(format!("FPS: {}", get_fps()).as_str(), 10.0, 20.0, 20., DARKGRAY);
    }
}

impl render::Render for GameScene {
    fn render(self: &Self, cell: &cell::Cell) {
        let ui = Ui{ caves_per: 0.7, invent_per: 0.3 };
        let leftpane_cell = cell::Cell::new((0.0, 0.0), (cell.x_most * ui.caves_per, cell.y_most));
        let rightpane_cell = cell::Cell::new((cell.x_most * ui.caves_per, 0.0), (cell.x_most, cell.y_most));

        clear_background(BLACK);

        self.cave.render(&leftpane_cell);
        GameScene::draw_seperator(ui.caves_per);
        self.inventory.render(&rightpane_cell);

        GameScene::render_fps();
    }
}

impl GameScene {
    pub fn logic(self: &mut Self) {
        self.cave.logic();
    }

    fn draw_seperator(horizontal_fraction: f32) {
        draw_line(
            horizontal_fraction * screen_width(),
            0.0,
            horizontal_fraction * screen_width(),
            screen_height(),
            2.0,
            LIGHTGRAY
        );
    }
}
