use crate::color::Color;
use crate::playground::{Playground, COLUMN_COUNT, ROW_COUNT};
use crate::tetrominos::{get_suffled_tetrominos, Tetromino};
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
    level: u32,
    is_game_over: bool,
    next_tetrominos: Vec<Box<dyn Tetromino>>,
    current_tetromino: Option<Box<dyn Tetromino>>,
    playground: Playground,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl TetrisGame {
    pub fn tick(&mut self) {
        if self.is_game_over {
            return;
        }
        let mut next = self.playground.clone();
        match &mut self.current_tetromino {
            None => {
                if self.next_tetrominos.is_empty() {
                    self.next_tetrominos = get_suffled_tetrominos();
                }
                let tetromino: Box<dyn Tetromino> = self.next_tetrominos.pop().unwrap();
                self.is_game_over = !tetromino.insert_into_playground(&mut next);
                self.current_tetromino = Some(tetromino);
            }
            Some(tetromino) => {
                if !tetromino.go_down(&mut next) {
                    self.current_tetromino = None;
                    let line_removed_count = next.remove_full_lines();
                    self.set_score(line_removed_count);
                }
            }
        }

        self.playground = next;
    }

    fn set_score(&mut self, line_removed_count: u8) {
        let coeff = match line_removed_count {
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => 0,
        };
        self.score += coeff * (self.level + 1);
    }

    pub fn new() -> TetrisGame {
        set_panic_hook();

        TetrisGame {
            score: 0,
            level: 0,
            is_game_over: false,
            current_tetromino: None,
            next_tetrominos: get_suffled_tetrominos(),
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

    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }

    pub fn level(&self) -> u32 {
        self.level
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

    pub fn rotate_clockwise(&mut self) {
        if let Some(tetromino) = &mut self.current_tetromino {
            let mut next = self.playground.clone();
            tetromino.rotate_clockwise(&mut next);
            self.playground = next;
        }
    }

    pub fn rotate_anticlockwise(&mut self) {
        if let Some(tetromino) = &mut self.current_tetromino {
            let mut next = self.playground.clone();
            tetromino.rotate_anticlockwise(&mut next);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_score_level0_0line() {
        // given
        let mut game = TetrisGame::new();
        game.level = 0;

        // when
        game.set_score(0);

        // then
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn set_score_level0_1line() {
        // given
        let mut game = TetrisGame::new();
        game.level = 0;

        // when
        game.set_score(1);

        // then
        assert_eq!(game.score(), 40);
    }

    #[test]
    fn set_score_level0_2lines() {
        // given
        let mut game = TetrisGame::new();
        game.level = 0;

        // when
        game.set_score(2);

        // then
        assert_eq!(game.score(), 100);
    }

    #[test]
    fn set_score_level0_3lines() {
        // given
        let mut game = TetrisGame::new();
        game.level = 0;

        // when
        game.set_score(3);

        // then
        assert_eq!(game.score(), 300);
    }

    #[test]
    fn set_score_level0_4lines() {
        // given
        let mut game = TetrisGame::new();
        game.level = 0;

        // when
        game.set_score(4);

        // then
        assert_eq!(game.score(), 1200);
    }

    #[test]
    fn set_score_level1_1line() {
        // given
        let mut game = TetrisGame::new();
        game.level = 1;

        // when
        game.set_score(1);

        // then
        assert_eq!(game.score(), 80);
    }

    #[test]
    fn set_score_level1_2lines() {
        // given
        let mut game = TetrisGame::new();
        game.level = 1;

        // when
        game.set_score(2);

        // then
        assert_eq!(game.score(), 200);
    }

    #[test]
    fn set_score_level1_3lines() {
        // given
        let mut game = TetrisGame::new();
        game.level = 1;

        // when
        game.set_score(3);

        // then
        assert_eq!(game.score(), 600);
    }

    #[test]
    fn set_score_level1_4lines() {
        // given
        let mut game = TetrisGame::new();
        game.level = 1;

        // when
        game.set_score(4);

        // then
        assert_eq!(game.score(), 2400);
    }
}
