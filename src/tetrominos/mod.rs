mod i;
mod j;
mod l;
mod o;
mod s;
mod t;
mod z;
use crate::color::Color;
use crate::playground::Playground;
use crate::playground::{COLUMN_COUNT, ROW_COUNT};
use rand::{prelude::SliceRandom, thread_rng};

const SQUARE_COUNT: usize = 4;

pub fn get_suffled_tetrominos() -> Vec<Box<dyn Tetromino>> {
    let mut tetrominos: Vec<Box<dyn Tetromino>> = vec![
        Box::new(i::I::new()),
        Box::new(j::J::new()),
        Box::new(l::L::new()),
        Box::new(o::O::new()),
        Box::new(s::S::new()),
        Box::new(t::T::new()),
        Box::new(z::Z::new()),
    ];
    tetrominos.shuffle(&mut thread_rng());
    tetrominos
}

pub trait Tetromino {
    fn new() -> Self
    where
        Self: Sized;
    fn insert_into_playground(&self, playground: &mut Playground) -> bool;
    fn go_down(&mut self, playground: &mut Playground) -> bool;
    fn go_right(&mut self, playground: &mut Playground) -> bool;
    fn go_left(&mut self, playground: &mut Playground) -> bool;
    fn go_bottom(&mut self, playground: &mut Playground) -> bool;
    fn rotate_clockwise(&mut self, playground: &mut Playground) -> bool;
    fn rotate_anticlockwise(&mut self, playground: &mut Playground) -> bool;
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

    fn insert_into_playground(&self, playground: &mut Playground) -> bool {
        let is_ok =
            (0..SQUARE_COUNT).all(|i| playground.get_squares()[self.get_square(i)] == Color::None);
        if !is_ok {
            return false;
        }
        (0..SQUARE_COUNT).for_each(|i| {
            playground.set_square(self.get_square(i), self.get_color());
        });
        true
    }

    fn can_go_down(&self, playground: &Playground) -> bool {
        (0..SQUARE_COUNT).fold(true, |acc, i| {
            if !acc || self.get_square(i) >= COLUMN_COUNT * (ROW_COUNT - 1) {
                return false;
            }
            let is_cell_bellow_free = playground.is_cell_bellow_free(self.get_square(i));
            let is_cell_bellow_used_by_this_tetromino =
                self.is_cell_used_by_this_tetromino(i, COLUMN_COUNT as i16);
            is_cell_bellow_free || is_cell_bellow_used_by_this_tetromino
        })
    }

    fn go_down(&mut self, playground: &mut Playground) -> bool {
        if !self.can_go_down(playground) {
            return false;
        }
        self.set_tetromino_on_new_offset(playground, COLUMN_COUNT as i16);
        true
    }

    fn go_right(&mut self, playground: &mut Playground) -> bool {
        let can_go_right = (0..SQUARE_COUNT).fold(true, |acc, i| {
            let is_cell_on_the_right_free =
                playground.is_cell_on_the_right_free(self.get_square(i));
            let is_cell_on_the_right_used_by_this_tetromino =
                self.is_cell_used_by_this_tetromino(i, 1);
            acc && (is_cell_on_the_right_free || is_cell_on_the_right_used_by_this_tetromino)
        });
        if !can_go_right {
            return false;
        }
        self.set_tetromino_on_new_offset(playground, 1);
        true
    }

    fn go_left(&mut self, playground: &mut Playground) -> bool {
        let can_go_left = (0..SQUARE_COUNT).fold(true, |acc, i| {
            let is_cell_on_the_left_free = playground.is_cell_on_the_left_free(self.get_square(i));
            let is_cell_on_the_left_used_by_this_tetromino =
                self.is_cell_used_by_this_tetromino(i, -1);
            acc && (is_cell_on_the_left_free || is_cell_on_the_left_used_by_this_tetromino)
        });
        if !can_go_left {
            return false;
        }
        self.set_tetromino_on_new_offset(playground, -1);
        true
    }

    fn go_bottom(&mut self, playground: &mut Playground) -> bool {
        let mut have_moved = false;
        // TODO: optimise this
        while self.can_go_down(playground) {
            have_moved = true;
            self.set_tetromino_on_new_offset(playground, COLUMN_COUNT as i16);
        }
        have_moved
    }
}
