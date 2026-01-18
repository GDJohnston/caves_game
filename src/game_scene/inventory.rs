use macroquad::prelude::*;

use crate::MAX_LENGTH;
use crate::render;
use crate::cell;

pub struct Inventory {
    // x/columns 4, y/rows 7
    pub rows: u16,
    pub columns: u16,
}

impl render::Render for Inventory {
    fn render(self: &Self, cell: &cell::Cell) {
        let x_len = cell.x_most - cell.x_least;
        let y_len = cell.y_most - cell.y_least;
        let game_size = x_len.min(y_len);

        let mut offset_x = (x_len - game_size) / 2.0 + 10.0;
        let mut offset_y = (y_len - game_size) / 2.0 + 10.0;

        let square_size = (x_len - offset_x * 2.0) / MAX_LENGTH as f32;

        let x_l2 = (MAX_LENGTH - self.columns) as f32;
        let y_l2 = (MAX_LENGTH - self.rows) as f32;

        offset_x += (x_l2/2.0) * square_size as f32;
        offset_y += (y_l2/2.0) * square_size as f32;

        let grid_start_x = cell.x_least + offset_x;
        let grid_start_y = cell.y_least + offset_y;

        // draw_line(
        //     cell.x_least + offset_x,
        //     cell.y_least + offset_y,
        //     cell.x_most - offset_x,
        //     cell.y_most - offset_y,
        //     2.0, RED);
        for i in 0..self.rows + 1 { // Horizontal
            draw_line(
                grid_start_x,
                grid_start_y + square_size * i as f32,
                cell.x_most - offset_x,
                grid_start_y + square_size * i as f32,
                2.0,
                LIGHTGRAY,
            );
        }

        for i in 0..self.columns + 1 { // Vertical
            draw_line(
                grid_start_x + square_size * i as f32,
                grid_start_y,
                grid_start_x + square_size * i as f32,
                cell.y_most - offset_y,
                2.0,
                LIGHTGRAY,
            );

        // for i in 0..self.rows + 1 { // Horizontal
        //     draw_line(
        //         cell.x_least + offset_x,
        //         cell.y_least + offset_y + square_size * i as f32,
        //         cell.x_most - offset_x,
        //         cell.y_least + offset_y + square_size * i as f32,
        //         2.,
        //         PINK,
        //     );
        // }

        // for i in 0..self.columns + 1 { // Vertical
        //     draw_line(
        //         cell.x_least + offset_x + square_size * i as f32,
        //         cell.y_least + offset_y,
        //         cell.x_least + offset_x + square_size * i as f32,
        //         cell.y_least + self.rows as f32 * square_size + offset_y,
        //         2.,
        //         ORANGE,
        //     );
        }
    }
}
