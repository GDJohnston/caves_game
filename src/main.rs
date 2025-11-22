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

#[derive(Default)]
struct Caves {
    offset_x:f32,
    offset_y:f32,
    square_size: f32,
}

struct Ui {
    caves_per: f32,
    invent_per: f32,
}
impl Caves {
    fn new(frame: &Frame, border_x: f32, border_y: f32) -> Self {
        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2.0 + border_x;
        let offset_y = (screen_height() - game_size) / 2.0 + border_y;
        let square_size = (screen_height() - offset_y * 2.0) / MAX_LENGTH as f32;

        Self { offset_x, offset_y, square_size }
    }
}

struct Frame {
    x_least: f32,
    y_least: f32,
    x_most: f32,
    y_most: f32,
}

#[macroquad::main("Caves")]
async fn main() {
    let x_length = rand::rng().random_range(1..=7);
    let y_length = rand::rng().random_range(1..=7);

    // let x_length = 5;
    // let y_length = 3;
    let ui = Ui{ caves_per: 0.7, invent_per: 0.3 };
    let screen_size = screen_width().min(screen_height());
    let global_margin = 10.0;
    let caves_frame = Frame { x_least: global_margin, y_least: global_margin, x_most: screen_size * ui.caves_per - global_margin, y_most: screen_size - global_margin };
    loop {
        render_caves(&caves_frame, x_length, y_length);
        // render_inventory(x_length, y_length);
        next_frame().await;
    }   
}

fn render_inventory(x_width: u32, y_width: u32) {
    clear_background(BLACK);
    
    let screen_size = screen_width().min(screen_height());
    let mut offset_x = (screen_width() - screen_size) / 2.0 + 10.0;
    let mut offset_y = (screen_height() - screen_size) / 2.0 + 10.0;
    let square_size = ((screen_height() - offset_y * 2.0) / MAX_LENGTH as f32) * 0.7;

    let x_l2 = (MAX_LENGTH - x_width) as f32;
    let y_l2 = (MAX_LENGTH - y_width) as f32;

    offset_x += (x_l2/2.0) * square_size as f32;
    offset_y += (y_l2/2.0) * square_size as f32;

    for i in 0..y_width + 1 { // Rows
        draw_line(
            offset_x,
            offset_y + square_size * i as f32,
            screen_width() - offset_x,
            offset_y + square_size * i as f32,
            2.,
            LIGHTGRAY,
        );
    }

    for i in 0..x_width + 1 { // Columns
        draw_line(
            offset_x + square_size * i as f32,
            offset_y,
            offset_x + square_size * i as f32,
            y_width as f32 * square_size + offset_y,
            2.,
            LIGHTGRAY,
        );
    }
}

fn render_caves(frame: &Frame, x_length: u32, y_length:u32) {
    clear_background(BLACK);

    // Seperator
    // draw_line(0.7 * screen_width(), 0.0, 0.7 * screen_width(), screen_height(), 2.0, RED);

    let caves: Caves = Caves::new(frame, 10.0, 10.0);
    draw_cave(&caves, x_length, y_length);

    let mut offset_x = caves.offset_x;
    let mut offset_y = caves.offset_y;
    let sq_size = caves.square_size;

    let x_l2 = (MAX_LENGTH - x_length) as f32;
    let y_l2 = (MAX_LENGTH - y_length) as f32;

    offset_x += (x_l2/2.0) * sq_size as f32;
    offset_y += (y_l2/2.0) * sq_size as f32;

    
    for i in 1..y_length { // Rows
        draw_line(
            offset_x,
            offset_y + sq_size * i as f32,
            screen_width() - offset_x,
            offset_y + sq_size * i as f32,
            2.,
            LIGHTGRAY,
        );
    }

    for i in 1..x_length { // Columns
        draw_line(
            (offset_x + sq_size * i as f32),
            offset_y,
            (offset_x + sq_size * i as f32),
            (y_length as f32 * sq_size + offset_y),
            2.,
            LIGHTGRAY,
        );
    }

}

fn draw_cave(caves: &Caves, x_length: u32, y_length:u32) {
    draw_tiles(caves, x_length, y_length);
}

fn draw_tiles(caves: &Caves, x_length: u32, y_length: u32) {
    let x_l2 = (MAX_LENGTH - x_length) as f32;
    let y_l2 = (MAX_LENGTH - y_length) as f32;

    let start_x = caves.offset_x + (x_l2/2.0) * caves.square_size as f32;
    let start_y = caves.offset_y + (y_l2/2.0) * caves.square_size as f32;
    
    let width = x_length as f32 * caves.square_size;
    let height = y_length as f32 * caves.square_size;
    draw_rectangle(start_x, start_y, width, height, DARKGRAY);
}
