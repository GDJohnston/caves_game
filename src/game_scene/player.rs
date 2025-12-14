use macroquad::prelude::*;

use crate::MAX_LENGTH;
use crate::cell;
use crate::render;
use crate::game_scene::point;

pub struct Player {
    pub pos: point::Point,
    pub cave_rows: u32,
    pub cave_columns: u32,
}

impl Player {
    pub fn move_player(&mut self, diff: point::Point) {
        self.pos += diff;

        if self.pos.0 < 0 {
            self.pos.0 = 0;
        }

        if self.pos.1 < 0 {
            self.pos.1 = 0;
        }
    }
}

impl render::Render for Player {
    fn render(self: &Self, cell: &cell::Cell) {
        let player = self;
        let game_size = cell.x_most.min(cell.y_most);
        let offset_x = (cell.x_most - game_size) / 2.0 + 10.0;
        let offset_y = (cell.y_most - game_size) / 2.0 + 10.0;
        let square_size = (cell.y_most - offset_y * 2.0) / MAX_LENGTH as f32;

        let x_l2 = (MAX_LENGTH - self.cave_rows) as f32;
        let y_l2 = (MAX_LENGTH - self.cave_columns) as f32;

        let grid_start_x = offset_x + (x_l2/2.0) * square_size as f32;
        let grid_start_y = offset_y + (y_l2/2.0) * square_size as f32;

        let font_size = 60.0;
        let text_size = measure_text("P", None, font_size as _, 1.0);


        let player_x = grid_start_x + square_size / 2.0 + player.pos.0 as f32 * square_size - text_size.width / 2.0;
        let player_y = grid_start_y + square_size / 2.0 + player.pos.1 as f32 * square_size + text_size.height / 2.0;

        draw_text("P", player_x, player_y, font_size, BEIGE);
    }
}