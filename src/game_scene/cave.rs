use macroquad::prelude::*;

use crate::MAX_LENGTH;
use crate::render;
use crate::cell;

pub struct Cave {
    pub rows: u32,
    pub columns: u32,
}

impl render::Render for Cave {
    fn render(self: &Self, cell: &cell::Cell) {
        draw_tiles(cell, self.rows, self.columns);
        draw_grid(cell, self.rows, self.columns);
}
}

fn draw_tiles(cell: &cell::Cell, cave_rows: u32, cave_columns: u32) {
    let game_size = cell.x_most.min(cell.y_most);
    let offset_x = (cell.x_most - game_size) / 2.0 + 10.0;
    let offset_y = (cell.y_most - game_size) / 2.0 + 10.0;
    let square_size = (cell.y_most - offset_y * 2.0) / MAX_LENGTH as f32;

    let x_l2 = (MAX_LENGTH - cave_rows) as f32;
    let y_l2 = (MAX_LENGTH - cave_columns) as f32;

    let start_x = offset_x + (x_l2/2.0) * square_size as f32;
    let start_y = offset_y + (y_l2/2.0) * square_size as f32;
    
    let width = cave_rows as f32 * square_size;
    let height = cave_columns as f32 * square_size;
    draw_rectangle(start_x, start_y, width, height, DARKGRAY);
}

fn draw_grid(cell: &cell::Cell, cave_rows: u32, cave_columns: u32) {
    let game_size = cell.x_most.min(cell.y_most);
    let mut offset_x = (cell.x_most - game_size) / 2.0 + 10.0;
    let mut offset_y = (cell.y_most - game_size) / 2.0 + 10.0;
    let square_size = (cell.y_most - offset_y * 2.0) / MAX_LENGTH as f32;

    let x_l2 = (MAX_LENGTH - cave_rows) as f32;
    let y_l2 = (MAX_LENGTH - cave_columns) as f32;

    offset_x += (x_l2/2.0) * square_size as f32;
    offset_y += (y_l2/2.0) * square_size as f32;

    
    for i in 1..cave_columns { // Rows
        draw_line(
            offset_x,
            offset_y + square_size * i as f32,
            cell.x_most - offset_x,
            offset_y + square_size * i as f32,
            2.0,
            LIGHTGRAY,
        );
    }

    for i in 1..cave_rows { // Columns
        draw_line(
            offset_x + square_size * i as f32,
            offset_y,
            offset_x + square_size * i as f32,
            cave_columns as f32 * square_size + offset_y,
            2.0,
            LIGHTGRAY,
        );
    }
}
