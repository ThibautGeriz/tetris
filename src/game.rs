use crate::color::Color;
use crate::playground::{Playground, COLUMN_COUNT, ROW_COUNT};
use crate::tetrominos::{get_random_tetromino, Tetromino};
use crate::utils::set_panic_hook;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct TetrisGame {
    score: u32,
    current_tetromino: Option<Box<dyn Tetromino>>,
    playground: Playground,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl TetrisGame {
    pub fn tick(&mut self) {
        let mut next = self.playground.clone();
        match &mut self.current_tetromino {
            None => {
                let tetromino: Box<dyn Tetromino> = get_random_tetromino(&mut next);
                self.current_tetromino = Some(tetromino);
            }
            Some(tetromino) => {
                if !tetromino.go_down(&mut next) {
                    self.current_tetromino = None;
                }
            }
        }
        if self.playground.is_last_line_full() {
            self.score += 100;
            (0..COLUMN_COUNT).for_each(|_| {
                // next.insert(0, Color::None);
                // next.remove(next.len() - 1);
            })
        }

        self.playground = next;
    }

    pub fn new() -> TetrisGame {
        set_panic_hook();

        TetrisGame {
            score: 0,
            current_tetromino: None,
            playground: Playground::new(),
        }
    }

    pub fn squares(&self) -> *const Color {
        self.playground.get_squares().as_ptr()
    }

    pub fn column_count(&self) -> u32 {
        COLUMN_COUNT as u32
    }

    pub fn row_count(&self) -> u32 {
        ROW_COUNT as u32
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn go_right(&mut self) {
        if let Some(tetromino) = &mut self.current_tetromino {
            let mut next = self.playground.clone();
            tetromino.go_right(&mut next);
            self.playground = next;
        }
    }

    pub fn go_left(&mut self) {
        if let Some(tetromino) = &mut self.current_tetromino {
            let mut next = self.playground.clone();
            tetromino.go_left(&mut next);
            self.playground = next;
        }
    }

    pub fn go_bottom(&mut self) {
        if let Some(tetromino) = &mut self.current_tetromino {
            let mut next = self.playground.clone();
            tetromino.go_bottom(&mut next);
            self.playground = next;
        }
    }
}

impl TetrisGame {
    pub fn get_tetromino(&self) -> &Option<Box<dyn Tetromino>> {
        &self.current_tetromino
    }

    pub fn get_playground(&self) -> &Playground {
        &self.playground
    }
}

impl Default for TetrisGame {
    fn default() -> Self {
        Self::new()
    }
}
