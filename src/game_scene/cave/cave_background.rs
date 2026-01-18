use crate::{MAX_LENGTH, cell, render};


use macroquad::prelude::*;

pub struct CaveBackground {
    pub rows: u16,
    pub columns: u16,
}

impl CaveBackground {
    pub fn new(rows: u16, columns: u16) -> Self {
        Self { rows, columns }
    }

    fn draw_tiles(self: &Self, cell: &cell::Cell) {
        let cell_size = cell.x_most.min(cell.y_most);
        let offset_x = (cell.x_most - cell_size) / 2.0 + 10.0;
        let offset_y = (cell.y_most - cell_size) / 2.0 + 10.0;

        let square_size = (cell.y_most - offset_y * 2.0) / MAX_LENGTH as f32;

        let x_l2 = (MAX_LENGTH - self.rows) as f32;
        let y_l2 = (MAX_LENGTH - self.columns) as f32;

        let start_x = offset_x + (x_l2/2.0) * square_size as f32;
        let start_y = offset_y + (y_l2/2.0) * square_size as f32;
        
        let width = self.rows as f32 * square_size;
        let height = self.columns as f32 * square_size;
        draw_rectangle(start_x, start_y, width, height, DARKGRAY);
}

fn draw_grid(self: &Self, cell: &cell::Cell) {
    let cell_size = cell.x_most.min(cell.y_most);
    let mut offset_x = (cell.x_most - cell_size) / 2.0 + 10.0;
    let mut offset_y = (cell.y_most - cell_size) / 2.0 + 10.0;
    let square_size = (cell.y_most - offset_y * 2.0) / MAX_LENGTH as f32;

    let x_l2 = (MAX_LENGTH - self.rows) as f32;
    let y_l2 = (MAX_LENGTH - self.columns) as f32;

    offset_x += (x_l2/2.0) * square_size as f32;
    offset_y += (y_l2/2.0) * square_size as f32;

    
    for i in 1..self.columns { // Rows
        draw_line(
            offset_x,
            offset_y + square_size * i as f32,
            cell.x_most - offset_x,
            offset_y + square_size * i as f32,
            2.0,
            LIGHTGRAY,
        );
    }

    for i in 1..self.rows { // Columns
        draw_line(
            offset_x + square_size * i as f32,
            offset_y,
            offset_x + square_size * i as f32,
            self.columns as f32 * square_size + offset_y,
            2.0,
            LIGHTGRAY,
        );
    }
}

}

impl render::Render for CaveBackground {
    fn render(self: &Self, cell: &cell::Cell) {
        self.draw_tiles(cell);
        self.draw_grid(cell);
    }
}
