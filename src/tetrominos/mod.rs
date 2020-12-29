mod o;
use crate::color::Color;
use crate::playground::Playground;

const SQUARE_COUNT: usize = 4;

pub fn get_random_tetromino() -> Box<dyn Tetromino> {
    Box::new(o::O::default())
}

pub trait Tetromino {
    fn get_color(&self) -> Color;
    fn get_squares(&self) -> [usize; SQUARE_COUNT];
    fn go_down(&mut self, playground: &mut Playground) -> bool;
    fn go_right(&mut self, playground: &mut Playground) -> bool;
    fn go_left(&mut self, playground: &mut Playground) -> bool;
    fn go_down_by(&mut self, length: usize) -> [usize; SQUARE_COUNT];
}

trait TetrominoPrivate {
    fn get_color(&self) -> Color;
    fn get_square(&self, index: usize) -> usize;
    fn set_square(&mut self, index: usize, value: usize);
    fn is_cell_used_by_this_tetromino(&self, index: usize, offset: i8) -> bool {
        (0..SQUARE_COUNT).any(|i| self.get_square(i) as i8 == self.get_square(index) as i8 + offset)
    }
    fn set_tetromino_on_new_offset(&mut self, playground: &mut Playground, offset: i8) {
        (0..SQUARE_COUNT).for_each(|i| playground.set_square(self.get_square(i), Color::None));
        (0..SQUARE_COUNT).for_each(|i| {
            self.set_square(i, (self.get_square(i) as i8 + offset) as usize);
            playground.set_square(self.get_square(i), self.get_color());
        });
    }
}
