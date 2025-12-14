use crate::cell;

pub trait Render {
    fn render(self: &Self, cell: &cell::Cell) {
        let _ = cell;
    }
}
