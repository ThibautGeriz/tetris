use crate::tetrominos::{get_random_tetromino, Tetromino};
use crate::utils::set_panic_hook;
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

#[wasm_bindgen]
pub struct TetrisGame {
    score: u32,
    current_tetromino: Option<Box<dyn Tetromino>>,
    squares: Vec<Color>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl TetrisGame {
    pub fn tick(&mut self) {
        let mut next = self.squares.clone();
        match &self.current_tetromino {
            None => {
                let tetromino: Box<dyn Tetromino> = get_random_tetromino();
                next[tetromino.get_squares()[0]] = tetromino.get_color();
                self.current_tetromino = Some(tetromino);
            }
            Some(tetromino) if self.is_next_cell_free(tetromino.get_squares()[0]) => {
                next[tetromino.get_squares()[0]] = Color::None;
                let color = tetromino.get_color();
                let new_index = self.current_tetromino.as_mut().unwrap().go_down()[0];
                next[new_index] = color;
            }
            _ => {
                self.current_tetromino = None;
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
        set_panic_hook();
        let squares = (0..COLUMN_COUNT * ROW_COUNT)
            .map(|_i| Color::None)
            .collect();

        TetrisGame {
            score: 0,
            current_tetromino: None,
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
        match &self.current_tetromino {
            Some(tetromino) if tetromino.get_squares()[0] % COLUMN_COUNT != COLUMN_COUNT - 1 => {
                let mut next = self.squares.clone();
                next[tetromino.get_squares()[0]] = Color::None;
                let color = tetromino.get_color();
                let new_index = self.current_tetromino.as_mut().unwrap().go_right()[0];
                next[new_index] = color;
                self.squares = next;
            }
            _ => {}
        }
    }

    pub fn go_left(&mut self) {
        match &self.current_tetromino {
            Some(tetromino) if tetromino.get_squares()[0] % COLUMN_COUNT > 0 => {
                let mut next = self.squares.clone();
                next[tetromino.get_squares()[0]] = Color::None;
                let color = tetromino.get_color();
                let new_index = self.current_tetromino.as_mut().unwrap().go_left()[0];
                next[new_index] = color;
                self.squares = next;
            }
            _ => {}
        }
    }

    pub fn go_bottom(&mut self) {
        match &self.current_tetromino {
            Some(tetromino) if tetromino.get_squares()[0] > 0 => {
                let mut next = self.squares.clone();
                let mut new_index = tetromino.get_squares()[0];
                let mut rows_to_go_down = 0;
                let color = tetromino.get_color();
                next[tetromino.get_squares()[0]] = Color::None;
                while self.is_next_cell_free(new_index) {
                    new_index += COLUMN_COUNT;
                    rows_to_go_down += 1;
                }
                let new_index = self
                    .current_tetromino
                    .as_mut()
                    .unwrap()
                    .go_down_by(rows_to_go_down)[0];
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

    pub fn get_tetromino(&self) -> &Option<Box<dyn Tetromino>> {
        &self.current_tetromino
    }
}

impl Default for TetrisGame {
    fn default() -> Self {
        Self::new()
    }
}
