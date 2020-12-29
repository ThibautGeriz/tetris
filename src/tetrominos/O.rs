use super::Tetromino;
use crate::game::Color;
use crate::game::COLUMN_COUNT;
use rand::{thread_rng, Rng};

pub struct O {
    squares: [usize; 4]
}

impl O {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let index = rng.gen_range(2, COLUMN_COUNT - 2);
        let mut squares = [0; 4];
        squares[0] = index;
        squares[1] = index + 1;
        squares[2] = index + COLUMN_COUNT;
        squares[3] = index + 1 + COLUMN_COUNT;
        O { squares  }
    }
}

impl Tetromino for O {
    fn get_color(&self) -> Color {
        Color::Yellow
    }

    fn get_squares(&self) -> [usize; 4] {
        self.squares
    }

    fn go_down(&mut self) -> [usize; 4] {
        // self.index += COLUMN_COUNT;
        self.squares
    }

    fn go_right(&mut self) -> [usize; 4] {
        // self.index += 1;
        self.squares
    }

    fn go_left(&mut self) -> [usize; 4] {
        // self.index -= 1;
        self.squares
    }

    fn go_down_by(&mut self, length: usize) -> [usize; 4] {
        // self.index += COLUMN_COUNT * length;
        self.squares
    }
}

impl Default for O {
    fn default() -> Self {
        Self::new()
    }
}
