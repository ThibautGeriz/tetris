use super::Tetromino;
use crate::game::Color;
use crate::game::COLUMN_COUNT;
use rand::{thread_rng, Rng};

pub struct O {
    pub index: usize,
}

impl O {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let index = rng.gen_range(1, COLUMN_COUNT - 1);
        O { index }
    }
}

impl Tetromino for O {
    fn get_color(&self) -> Color {
        Color::Yellow
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn go_down(&mut self) -> usize {
        self.index += COLUMN_COUNT;
        self.index
    }

    fn go_right(&mut self) -> usize {
        self.index += 1;
        self.index
    }

    fn go_left(&mut self) -> usize {
        self.index -= 1;
        self.index
    }

    fn go_down_by(&mut self, length: usize) -> usize {
        self.index += COLUMN_COUNT * length;
        self.index
    }
}

impl Default for O {
    fn default() -> Self {
        Self::new()
    }
}
