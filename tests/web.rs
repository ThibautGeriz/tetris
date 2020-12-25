//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_tetris;
use wasm_tetris::Square;
use wasm_tetris::TetrisGame;
use wasm_tetris::HEIGHT;
use wasm_tetris::WIDTH;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn empty_game() -> Vec<Square> {
    (0..200).map(|_i| Square::Free).collect()
}

#[wasm_bindgen_test]
pub fn init() {
    // when
    let game = TetrisGame::new();

    // then
    assert_eq!(game.get_squares(), &empty_game());
}

#[wasm_bindgen_test]
pub fn after_first_tick() {
    // given
    let mut game = TetrisGame::new();

    // when
    game.tick();

    // then
    let first_line = &game.get_squares()[0..WIDTH];
    let used_square_count = first_line
        .iter()
        .filter(|s| s == &&Square::Occupied)
        .count();
    assert_eq!(used_square_count, 1);
}

#[wasm_bindgen_test]
pub fn after_second_tick() {
    // given
    let mut game = TetrisGame::new();

    // when
    game.tick();
    game.tick();

    // then
    let first_line = &game.get_squares()[0..WIDTH];
    let used_square_count = first_line
        .iter()
        .filter(|s| s == &&Square::Occupied)
        .count();
    assert_eq!(used_square_count, 0);

    let start_index: usize = WIDTH;
    let end_index: usize = (WIDTH) * 2;
    let second_line = &game.get_squares()[start_index..end_index];
    let used_square_count = second_line
        .iter()
        .filter(|s| s == &&Square::Occupied)
        .count();
    assert_eq!(used_square_count, 1);
}

#[wasm_bindgen_test]
pub fn go_bottom() {
    // given
    let mut game = TetrisGame::new();
    game.tick();
    let index = game.current_square.unwrap();

    // when
    game.go_bottom();

    // then
    let start_index: usize = WIDTH * (HEIGHT - 1);
    let end_index: usize = WIDTH * HEIGHT;
    let last_line = &game.get_squares()[start_index..end_index];
    let used_square_count = last_line.iter().filter(|s| s == &&Square::Occupied).count();
    assert_eq!(game.current_square.unwrap(), index + 190);
    assert_eq!(used_square_count, 1);
}

#[wasm_bindgen_test]
pub fn go_right() {
    // given
    let mut game = TetrisGame::new();
    game.tick();
    let index = game.current_square.unwrap();

    // when
    game.go_right();

    // then
    assert_eq!(game.current_square.unwrap(), index + 1);
    assert_eq!(game.get_squares()[index + 1], Square::Occupied);
    assert_eq!(game.get_squares()[index], Square::Free);
}

#[wasm_bindgen_test]
pub fn go_left() {
    // given
    let mut game = TetrisGame::new();
    game.tick();
    let index = game.current_square.unwrap();

    // when
    game.go_left();

    // then
    assert_eq!(game.current_square.unwrap(), index - 1);
    assert_eq!(game.get_squares()[index - 1], Square::Occupied);
    assert_eq!(game.get_squares()[index], Square::Free);
}
