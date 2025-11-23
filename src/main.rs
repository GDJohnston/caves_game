use ::rand::Rng;
use ::rand;
use macroquad::prelude::*;

const MAX_LENGTH:u32 = 7;
    
struct ColourScheme {
    background_colour: Color,
    tile_colour: Color,
    seperator_colour: Color,
}

const DARK_SCHEME: ColourScheme = ColourScheme{ background_colour: BLACK, tile_colour: DARKGRAY, seperator_colour: LIGHTGRAY };

struct Ui {
    caves_per: f32,
    invent_per: f32,
}

struct Cell {
    x_least: f32,
    y_least: f32,
    x_most: f32,
    y_most: f32,
}

impl Cell {
    fn draw(&self, colour: Color) {
        draw_rectangle(
            self.x_least,
            self.y_least,
            self.x_most,
            self.y_most,
            colour
        );
    }
}

#[macroquad::main("Caves")]
async fn main() {
    // let x_length = rand::rng().random_range(1..=7);
    // let y_length = rand::rng().random_range(1..=7);

    let x_length = 5;
    let y_length = 3;
    let ui = Ui{ caves_per: 0.7, invent_per: 0.3 };
    loop {
        let global_cell = Cell {x_least: 0.0, y_least: 0.0, x_most: screen_width(), y_most: screen_height()};
        let caves_cell = Cell { x_least: 0.0, y_least: 0.0, x_most: global_cell.x_most * ui.caves_per, y_most: global_cell.y_most};
        let rightpane_cell = Cell { x_least: global_cell.x_most * ui.caves_per, y_least: 0.0, x_most: global_cell.x_most, y_most: global_cell.y_most};

        clear_background(BLACK);

        // Seperator
        draw_line(0.7 * screen_width(), 0.0, 0.7 * screen_width(), screen_height(), 2.0, LIGHTGRAY);
        render_caves(&caves_cell, x_length, y_length);
        render_inventory(&rightpane_cell, 3, 5);
        next_frame().await;
    }   
}

fn render_inventory(cell: &Cell, x_length: u32, y_length: u32) {
    let x_len = cell.x_most - cell.x_least;
    let y_len = cell.y_most - cell.y_least;
    let game_size = x_len.min(y_len);

    let mut offset_x = (x_len - game_size) / 2.0 + 10.0;
    let mut offset_y = (y_len - game_size) / 2.0 + 10.0;

    let square_size = (x_len - offset_x * 2.0) / MAX_LENGTH as f32;

    let x_l2 = (MAX_LENGTH - x_length) as f32;
    let y_l2 = (MAX_LENGTH - y_length) as f32;

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
    for i in 0..y_length + 1 { // Horizontal
        draw_line(
            grid_start_x,
            grid_start_y + square_size * i as f32,
            cell.x_most - offset_x,
            grid_start_y + square_size * i as f32,
            2.0,
            LIGHTGRAY,
        );
    }

    for i in 0..x_length + 1 { // Vertical
        draw_line(
            grid_start_x + square_size * i as f32,
            grid_start_y,
            grid_start_x + square_size * i as f32,
            cell.y_most - offset_y,
            2.0,
            LIGHTGRAY,
        );

    // for i in 0..y_length + 1 { // Horizontal
    //     draw_line(
    //         cell.x_least + offset_x,
    //         cell.y_least + offset_y + square_size * i as f32,
    //         cell.x_most - offset_x,
    //         cell.y_least + offset_y + square_size * i as f32,
    //         2.,
    //         PINK,
    //     );
    // }

    // for i in 0..x_length + 1 { // Vertical
    //     draw_line(
    //         cell.x_least + offset_x + square_size * i as f32,
    //         cell.y_least + offset_y,
    //         cell.x_least + offset_x + square_size * i as f32,
    //         cell.y_least + y_length as f32 * square_size + offset_y,
    //         2.,
    //         ORANGE,
    //     );
    }
}

fn render_caves(cell: &Cell, x_length: u32, y_length:u32) {
    draw_cave(cell, x_length, y_length);
}

fn draw_cave(cell: &Cell, x_length: u32, y_length:u32) {
    draw_tiles(cell, x_length, y_length);
    draw_grid(cell, x_length, y_length);
}

fn draw_tiles(cell: &Cell, x_length: u32, y_length: u32) {
    let game_size = cell.x_most.min(cell.y_most);
    let offset_x = (cell.x_most - game_size) / 2.0 + 10.0;
    let offset_y = (cell.y_most - game_size) / 2.0 + 10.0;
    let square_size = (cell.y_most - offset_y * 2.0) / MAX_LENGTH as f32;

    let x_l2 = (MAX_LENGTH - x_length) as f32;
    let y_l2 = (MAX_LENGTH - y_length) as f32;

    let start_x = offset_x + (x_l2/2.0) * square_size as f32;
    let start_y = offset_y + (y_l2/2.0) * square_size as f32;
    
    let width = x_length as f32 * square_size;
    let height = y_length as f32 * square_size;
    draw_rectangle(start_x, start_y, width, height, DARKGRAY);
}

fn draw_grid(cell: &Cell, x_length: u32, y_length: u32) {
    let game_size = cell.x_most.min(cell.y_most);
    let mut offset_x = (cell.x_most - game_size) / 2.0 + 10.0;
    let mut offset_y = (cell.y_most - game_size) / 2.0 + 10.0;
    let square_size = (cell.y_most - offset_y * 2.0) / MAX_LENGTH as f32;

    let x_l2 = (MAX_LENGTH - x_length) as f32;
    let y_l2 = (MAX_LENGTH - y_length) as f32;

    offset_x += (x_l2/2.0) * square_size as f32;
    offset_y += (y_l2/2.0) * square_size as f32;

    
    for i in 1..y_length { // Rows
        draw_line(
            offset_x,
            (offset_y + square_size * i as f32),
            (cell.x_most - offset_x),
            (offset_y + square_size * i as f32),
            2.,
            LIGHTGRAY,
        );
    }

    for i in 1..x_length { // Columns
        draw_line(
            (offset_x + square_size * i as f32),
            offset_y,
            (offset_x + square_size * i as f32),
            (y_length as f32 * square_size + offset_y),
            2.,
            LIGHTGRAY,
        );
    }
}
