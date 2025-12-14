use macroquad::prelude::*;

pub struct Cell {
    pub x_least: f32,
    pub y_least: f32,
    pub x_most: f32,
    pub y_most: f32,
}

impl Cell {
    pub fn new(least: (f32, f32), most: (f32, f32)) -> Self {
        Self {
            x_least: least.0 as f32,
            y_least: least.1 as f32,
            x_most: most.0 as f32,
            y_most: most.1 as f32
        }
    }
}

impl Cell {
    pub fn _debug_render(&self, colour: Color) {
        draw_rectangle(
            self.x_least,
            self.y_least,
            self.x_most,
            self.y_most,
            colour
        );
    }
}