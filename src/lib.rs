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
    width: u32,
    height: u32,
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
                let index = rng.gen_range(0, self.width) as usize;
                self.current_square = Some(index);
                next[index] = Square::Occupied;
                log!("New square at {}", index);
            }
            Some(index) if self.is_next_cell_free(index) => {
                next[index as usize] = Square::Free;
                let new_index = index + self.width as usize;
                next[new_index] = Square::Occupied;
                self.current_square = Some(new_index);
                log!("Square goes down from {} to {}", index, new_index);
            }
            Some(index) => {
                self.current_square = None;
                log!("Square stops at {}", index);
            }
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
            current_square: None,
            height,
            squares,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn squares(&self) -> *const Square {
        self.squares.as_ptr()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl fmt::Display for TetrisGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.squares.as_slice().chunks(self.width as usize) {
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
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn is_next_cell_free(&self, index: usize) -> bool {
        let max_index_to_go_down = self.width as usize * (self.height as usize - 1);
        if index >= max_index_to_go_down {
            return false;
        }
        let new_index = index + self.width as usize;
        self.squares[new_index] == Square::Free
    }
}

impl Default for TetrisGame {
    fn default() -> Self {
        Self::new()
    }
}
