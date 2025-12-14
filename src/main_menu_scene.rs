use macroquad::prelude::*;

use crate::Scene;
use crate::cell;
use crate::render;

pub struct MainMenu {

}

impl render::Render for MainMenu {
    fn render(self: &Self, cell: &cell::Cell) {
        clear_background(BLACK);
        let text = "Caves\nPress [enter] to begin.";
        let font_size = 30.;
        let text_size = measure_text(text, None, font_size as _, 1.0);

        draw_multiline_text(
            text,
            cell.x_most / 2. - text_size.width / 2.,
            cell.y_most / 2. + text_size.height / 2.,
            font_size,
            None,
            BEIGE,
        );
    }
}

impl MainMenu {
    pub fn logic(self: &Self, scene: &mut Scene) {
        if is_key_down(KeyCode::Enter) {
            *scene = Scene::Game;
        }
    }
}