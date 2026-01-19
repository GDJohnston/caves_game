use macroquad::prelude::*;

use crate::MAX_LENGTH;
use crate::cell;
use crate::render;
use crate::game_scene::point;

const UP: point::Point = point::Point(0, -1);
const DOWN: point::Point = point::Point(0, 1);
const RIGHT: point::Point = point::Point(1, 0);
const LEFT: point::Point = point::Point(-1, 0);

pub struct Monster {
    pub pos: point::Point,
    pub cave_rows: u16,
    pub cave_columns: u16,
}

impl Monster {
    pub fn new(cave_rows: u16, cave_columns: u16) -> Self {
        let pos = point::Point((cave_rows - 1) as i16, (cave_columns - 1) as i16);
        Self { pos, cave_rows, cave_columns }
    }

    pub fn logic(self: &mut Self) {
        if is_key_pressed(KeyCode::I) {
            self.move_monster(UP);
        } else if is_key_pressed(KeyCode::J) {
            self.move_monster(LEFT);
        } else if is_key_pressed(KeyCode::K) {
            self.move_monster(DOWN);
        } else if is_key_pressed(KeyCode::L) {
            self.move_monster(RIGHT);
        }
    }
    
    fn move_monster(&mut self, diff: point::Point) {
        self.pos += diff;

        let min = point::Point(0,0);
        let max = point::Point (
            (self.cave_rows - 1) as i16,
            (self.cave_columns - 1) as i16,
        );
        self.pos = self.pos.clamp(min, max);
    }

    pub fn _render_pos(&self) {
        draw_text(format!("POS: {:?}", self.pos).as_str(), 10.0, 40.0, 20., DARKGRAY);
    }
}

impl render::Render for Monster {
    fn render(self: &Self, cell: &cell::Cell) {
        let player = self;
        let cell_size = cell.x_most.min(cell.y_most);
        let offset_x = (cell.x_most - cell_size) / 2.0 + 10.0;
        let offset_y = (cell.y_most - cell_size) / 2.0 + 10.0;
        let square_size = (cell.y_most - offset_y * 2.0) / MAX_LENGTH as f32;

        let x_l2 = (MAX_LENGTH - self.cave_rows) as f32;
        let y_l2 = (MAX_LENGTH - self.cave_columns) as f32;

        let grid_start_x = offset_x + (x_l2/2.0) * square_size as f32;
        let grid_start_y = offset_y + (y_l2/2.0) * square_size as f32;

        let font_size = 60.0;
        let text_size = measure_text("M", None, font_size as _, 1.0);


        let player_x = grid_start_x + square_size / 2.0 + player.pos.0 as f32 * square_size - text_size.width / 2.0;
        let player_y = grid_start_y + square_size / 2.0 + player.pos.1 as f32 * square_size + text_size.height / 2.0;

        draw_text("M", player_x, player_y, font_size, BEIGE);
        player._render_pos();
    }
}