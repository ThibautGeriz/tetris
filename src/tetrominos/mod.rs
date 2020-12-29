mod o;
use crate::game::Color;

pub fn get_random_tetromino() -> Box<dyn Tetromino> {
    Box::new(o::O::new())
}

pub trait Tetromino {
    fn get_color(&self) -> Color;
    fn get_squares(&self) -> [usize; 4];
    fn go_down(&mut self) -> [usize; 4];
    fn go_right(&mut self) -> [usize; 4];
    fn go_left(&mut self) -> [usize; 4];
    fn go_down_by(&mut self, length: usize) -> [usize; 4];
}
