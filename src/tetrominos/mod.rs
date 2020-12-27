mod o;
use crate::game::Color;

pub fn get_random_tetromino() -> Box<dyn Tetromino> {
    Box::new(o::O::new())
}

pub trait Tetromino {
    fn get_color(&self) -> Color;
    fn get_index(&self) -> usize;
    fn go_down(&mut self) -> usize;
    fn go_right(&mut self) -> usize;
    fn go_left(&mut self) -> usize;
    fn go_down_by(&mut self, length: usize) -> usize;
}
