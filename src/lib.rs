mod utils;

use rand::Rng;
use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Free = 0,
    Occupied = 1,
}

#[wasm_bindgen]
pub struct TetrisGame {
    width: usize,
    height: usize,
    score: u32,
    current_square: Option<usize>,
    squares: Vec<Square>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl TetrisGame {
    pub fn tick(&mut self) {
        let mut next = self.squares.clone();
        match self.current_square {
            None => {
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0, self.width);
                self.current_square = Some(index);
                next[index] = Square::Occupied;
            }
            Some(index) if self.is_next_cell_free(index) => {
                next[index] = Square::Free;
                let new_index = index + self.width;
                next[new_index] = Square::Occupied;
                self.current_square = Some(new_index);
            }
            _ => {
                self.current_square = None;
            }
        }
        if self.is_last_line_full() {
            self.score += 100;
            (0..self.width).for_each(|_| {
                next.insert(0, Square::Free);
                next.remove(next.len() - 1);
            })
        }

        self.squares = next;
    }

    pub fn new() -> TetrisGame {
        utils::set_panic_hook();
        let width = 10;
        let height = 20;

        let squares = (0..width * height).map(|_i| Square::Free).collect();

        TetrisGame {
            width,
            score: 0,
            current_square: None,
            height,
            squares,
        }
    }

    pub fn squares(&self) -> *const Square {
        self.squares.as_ptr()
    }

    pub fn width(&self) -> u32 {
        self.width as u32
    }

    pub fn height(&self) -> u32 {
        self.height as u32
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn go_right(&mut self) {
        match self.current_square {
            Some(index) if index % self.width != self.width - 1 => {
                let mut next = self.squares.clone();
                let new_index = index + 1;
                self.current_square = Some(new_index);
                next[index] = Square::Free;
                next[new_index] = Square::Occupied;
                self.squares = next;
            }
            _ => {}
        }
    }

    pub fn go_left(&mut self) {
        match self.current_square {
            Some(index) if index % self.width > 0 => {
                let mut next = self.squares.clone();
                let new_index = index - 1;
                self.current_square = Some(new_index);
                next[index] = Square::Free;
                next[new_index] = Square::Occupied;
                self.squares = next;
            }
            _ => {}
        }
    }

    pub fn go_bottom(&mut self) {
        match self.current_square {
            Some(index) if index > 0 => {
                let mut next = self.squares.clone();
                let mut new_index = index;
                while self.is_next_cell_free(new_index) {
                    new_index += self.width;
                }
                self.current_square = Some(new_index);
                next[index] = Square::Free;
                next[new_index] = Square::Occupied;
                self.squares = next;
            }
            _ => {}
        }
    }
}

impl fmt::Display for TetrisGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.squares.as_slice().chunks(self.width) {
            for &square in line {
                let symbol = if square == Square::Free { ' ' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl TetrisGame {
    #[allow(dead_code)]
    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    fn is_next_cell_free(&self, index: usize) -> bool {
        let max_index_to_go_down = self.width * (self.height - 1);
        if index >= max_index_to_go_down {
            return false;
        }
        let new_index = index + self.width;
        self.squares[new_index] == Square::Free
    }

    fn is_last_line_full(&self) -> bool {
        let start_index = self.width * (self.height - 1);
        self.squares[start_index..]
            .iter()
            .all(|&x| x == Square::Occupied)
    }
}

impl Default for TetrisGame {
    fn default() -> Self {
        Self::new()
    }
}
