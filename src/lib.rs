mod utils;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub const COLUMN_COUNT: usize = 10;
pub const ROW_COUNT: usize = 20;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    None = 0,   // Empty
    Cyan = 1,   // I
    Yellow = 2, // O
    Purple = 3, // T
    Green = 4,  // S
    Red = 5,    // Z
    Blue = 6,   // J
    Orange = 7, // L
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0, 7) {
            0 => Color::Cyan,
            1 => Color::Yellow,
            2 => Color::Purple,
            3 => Color::Green,
            4 => Color::Red,
            5 => Color::Blue,
            _ => Color::Orange,
        }
    }
}

pub struct Tetromino {
    pub index: usize,
    pub color: Color,
}

impl Tetromino {
    fn new() -> Tetromino {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(1, COLUMN_COUNT - 1);
        let color: Color = rand::random();
        Tetromino { index, color }
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
}

#[wasm_bindgen]
pub struct TetrisGame {
    score: u32,
    current_square: Option<Tetromino>,
    squares: Vec<Color>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl TetrisGame {
    pub fn tick(&mut self) {
        let mut next = self.squares.clone();
        match self.current_square {
            None => {
                let tetromino = Tetromino::new();
                next[tetromino.index] = tetromino.color;
                self.current_square = Some(tetromino);
            }
            Some(Tetromino { index, color }) if self.is_next_cell_free(index) => {
                next[index] = Color::None;
                let new_index = self.current_square.as_mut().unwrap().go_down();
                next[new_index] = color;
            }
            _ => {
                self.current_square = None;
            }
        }
        if self.is_last_line_full() {
            self.score += 100;
            (0..COLUMN_COUNT).for_each(|_| {
                next.insert(0, Color::None);
                next.remove(next.len() - 1);
            })
        }

        self.squares = next;
    }

    pub fn new() -> TetrisGame {
        utils::set_panic_hook();
        let squares = (0..COLUMN_COUNT * ROW_COUNT)
            .map(|_i| Color::None)
            .collect();

        TetrisGame {
            score: 0,
            current_square: None,
            squares,
        }
    }

    pub fn squares(&self) -> *const Color {
        self.squares.as_ptr()
    }

    pub fn width(&self) -> u32 {
        COLUMN_COUNT as u32
    }

    pub fn height(&self) -> u32 {
        ROW_COUNT as u32
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn go_right(&mut self) {
        match self.current_square {
            Some(Tetromino { index, color }) if index % COLUMN_COUNT != COLUMN_COUNT - 1 => {
                let mut next = self.squares.clone();
                let new_index = self.current_square.as_mut().unwrap().go_right();
                next[new_index] = color;
                next[index] = Color::None;
                self.squares = next;
            }
            _ => {}
        }
    }

    pub fn go_left(&mut self) {
        match self.current_square {
            Some(Tetromino { index, color }) if index % COLUMN_COUNT > 0 => {
                let mut next = self.squares.clone();
                let new_index = self.current_square.as_mut().unwrap().go_left();
                next[new_index] = color;
                next[index] = Color::None;
                self.squares = next;
            }
            _ => {}
        }
    }

    pub fn go_bottom(&mut self) {
        match self.current_square {
            Some(Tetromino { index, color }) if index > 0 => {
                let mut next = self.squares.clone();
                let mut new_index = index;
                while self.is_next_cell_free(new_index) {
                    new_index += COLUMN_COUNT;
                }
                self.current_square = Some(Tetromino {
                    index: new_index,
                    color,
                });
                next[index] = Color::None;
                next[new_index] = color;
                self.squares = next;
            }
            _ => {}
        }
    }
}

impl fmt::Display for TetrisGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.squares.as_slice().chunks(COLUMN_COUNT) {
            for &square in line {
                let symbol = if square == Color::None { ' ' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl TetrisGame {
    fn is_next_cell_free(&self, index: usize) -> bool {
        let max_index_to_go_down = COLUMN_COUNT * (ROW_COUNT - 1);
        if index >= max_index_to_go_down {
            return false;
        }
        let new_index = index + COLUMN_COUNT;
        self.squares[new_index] == Color::None
    }

    fn is_last_line_full(&self) -> bool {
        let start_index = COLUMN_COUNT * (ROW_COUNT - 1);
        self.squares[start_index..]
            .iter()
            .all(|&x| x != Color::None)
    }

    pub fn get_squares(&self) -> &Vec<Color> {
        &self.squares
    }

    pub fn get_tetromino(&self) -> &Option<Tetromino> {
        &self.current_square
    }
}

impl Default for TetrisGame {
    fn default() -> Self {
        Self::new()
    }
}
