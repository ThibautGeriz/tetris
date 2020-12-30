//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_tetris;
use wasm_tetris::color::Color;
use wasm_tetris::game::TetrisGame;
use wasm_tetris::playground::COLUMN_COUNT;
use wasm_tetris::playground::ROW_COUNT;
use wasm_tetris::tetrominos::Tetromino;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn empty_game() -> Vec<Color> {
    (0..200).map(|_i| Color::None).collect()
}

#[wasm_bindgen_test]
pub fn init() {
    // when
    let game = TetrisGame::new();

    // then
    assert_eq!(game.get_playground().get_squares(), &empty_game());
}

#[wasm_bindgen_test]
pub fn after_first_tick() {
    // given
    let mut game = TetrisGame::new();

    // when
    game.tick();

    // then
    let first_line = &game.get_playground().get_squares()[0..COLUMN_COUNT];
    let used_square_count = first_line.iter().filter(|s| s != &&Color::None).count();
    assert!(used_square_count >= 1);
}

#[wasm_bindgen_test]
pub fn after_second_tick() {
    // given
    let mut game = TetrisGame::new();

    // when
    game.tick();
    game.tick();

    // then

    let start_index: usize = COLUMN_COUNT;
    let end_index: usize = (COLUMN_COUNT) * 2;
    let second_line = &game.get_playground().get_squares()[start_index..end_index];
    let used_square_count = second_line.iter().filter(|s| s != &&Color::None).count();
    assert!(used_square_count >= 1);
}

#[wasm_bindgen_test]
pub fn go_bottom() {
    // given
    let mut game = TetrisGame::new();
    game.tick();

    // when
    game.go_bottom();

    // then
    let start_index: usize = COLUMN_COUNT * (ROW_COUNT - 1);
    let end_index: usize = COLUMN_COUNT * ROW_COUNT;
    let last_line = &game.get_playground().get_squares()[start_index..end_index];
    let used_square_count = last_line.iter().filter(|s| s != &&Color::None).count();
    assert!(used_square_count >= 1);
}
