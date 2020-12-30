mod o;
use crate::color::Color;
use crate::playground::Playground;

const SQUARE_COUNT: usize = 4;

pub fn get_random_tetromino(playground: &mut Playground) -> Box<dyn Tetromino> {
    Box::new(o::O::new(playground))
}

pub trait Tetromino {
    fn go_down(&mut self, playground: &mut Playground) -> bool;
    fn go_right(&mut self, playground: &mut Playground) -> bool;
    fn go_left(&mut self, playground: &mut Playground) -> bool;
    fn go_bottom(&mut self, playground: &mut Playground) -> bool;
}

trait TetrominoCommon {
    fn get_color(&self) -> Color;
    fn get_square(&self, index: usize) -> usize;
    fn set_square(&mut self, index: usize, value: usize);
    fn is_cell_used_by_this_tetromino(&self, index: usize, offset: i16) -> bool {
        (0..SQUARE_COUNT)
            .any(|i| self.get_square(i) as i16 == self.get_square(index) as i16 + offset)
    }
    fn set_tetromino_on_new_offset(&mut self, playground: &mut Playground, offset: i16) {
        (0..SQUARE_COUNT).for_each(|i| playground.set_square(self.get_square(i), Color::None));
        (0..SQUARE_COUNT).for_each(|i| {
            self.set_square(i, (self.get_square(i) as i16 + offset) as usize);
            playground.set_square(self.get_square(i), self.get_color());
        });
    }
}
