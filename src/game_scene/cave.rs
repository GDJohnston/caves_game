mod cave_background;
mod player;
mod monster;

use macroquad::color::{Color, DARKGRAY};
use macroquad::text::draw_text;
use ::rand::Rng;
use ::rand;

use crate::{MAX_LENGTH, cell};
use crate::render::Render;

pub struct Cave {
    rows: u16,
    columns: u16,
    player: player::Player,
    monster: monster::Monster,
    cave_background: cave_background::CaveBackground,
}

impl Cave {
    pub fn new() -> Self {
        let rows: u16 = rand::rng().random_range(1..=7);
        let columns: u16 = rand::rng().random_range(1..=7);

        let cave_background = cave_background::CaveBackground::new(rows, columns);
        let player = player::Player::new(rows, columns);
        let monster = monster::Monster::new(rows, columns);
        Self {rows, columns, player, monster, cave_background}
    }

    pub fn logic(self: &mut Self) {
        self.player.logic();
        self.monster.logic();
    }

    fn make_cave_cell(self: &Self, cell: &cell::Cell) -> cell::Cell {
        let cell_min = cell.x_most.min(cell.y_most); // find the shortest side
        
        // Add 10 padding to both and additional padding to longest side
        let offset_x = (cell.x_most - cell_min) / 2.0 + 10.0;
        let offset_y = (cell.y_most - cell_min) / 2.0 + 10.0;

        // Calculate square size
        let square_size = (cell.y_most - offset_y * 2.0) / MAX_LENGTH as f32;

        // draw_text(format!("y_most: {:?}", cell.y_most).as_str(), 10.0, 40.0, 20., DARKGRAY);
        // draw_text(format!("offset_y: {:?}", offset_y).as_str(), 10.0, 60.0, 20., DARKGRAY);
        // draw_text(format!("square_size: {:?}", square_size).as_str(), 10.0, 80.0, 20., DARKGRAY);


        // Find the missing square count for each side
        let missing_x = (MAX_LENGTH - self.rows) as f32;
        let missing_y = (MAX_LENGTH - self.columns) as f32;

        // Starting coords for the cell
        let start_x = offset_x + (missing_x/2.0) * square_size as f32;
        let start_y = offset_y + (missing_y/2.0) * square_size as f32;

        let width = self.rows as f32 * square_size;
        let height = self.columns as f32 * square_size;

        let least = (start_x, start_y);
        let most = (width, height);
        cell::Cell::new(least, most)
    }
}

impl Render for Cave {
    fn render(self: &Self, cell: &cell::Cell) {
        let cave_cell = self.make_cave_cell(cell);
        self.cave_background.render(&cell);
        self.player.render(&cell);
        self.monster.render(&cell);

        // cave_cell._debug_render(Color::new(0.5,0.0,0.0,0.5));
    }
}
