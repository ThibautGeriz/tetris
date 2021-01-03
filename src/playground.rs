use crate::color::Color;
use std::fmt;

pub const COLUMN_COUNT: usize = 10;
pub const ROW_COUNT: usize = 20;

#[derive(Clone)]
pub struct Playground(Vec<Color>);

impl Playground {
    pub fn new() -> Playground {
        let squares = (0..COLUMN_COUNT * ROW_COUNT)
            .map(|_i| Color::None)
            .collect();
        Playground(squares)
    }

    pub fn is_cell_bellow_free(&self, index: usize) -> bool {
        let max_index_to_go_down = COLUMN_COUNT * (ROW_COUNT - 1);
        if index >= max_index_to_go_down {
            return false;
        }
        let new_index = index + COLUMN_COUNT;
        self.0[new_index] == Color::None
    }

    pub fn is_cell_on_the_right_free(&self, index: usize) -> bool {
        if index % COLUMN_COUNT == COLUMN_COUNT - 1 {
            return false;
        }
        let new_index = index + 1;
        self.0[new_index] == Color::None
    }

    pub fn is_cell_on_the_left_free(&self, index: usize) -> bool {
        if index % COLUMN_COUNT == 0 {
            return false;
        }
        let new_index = index - 1;
        self.0[new_index] == Color::None
    }

    pub fn is_last_line_full(&self) -> bool {
        let start_index = COLUMN_COUNT * (ROW_COUNT - 1);
        self.0[start_index..].iter().all(|&x| x != Color::None)
    }

    pub fn get_squares(&self) -> &Vec<Color> {
        &self.0
    }

    pub fn set_square(&mut self, index: usize, color: Color) {
        self.0[index] = color;
    }
}

impl fmt::Display for Playground {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.0.as_slice().chunks(COLUMN_COUNT) {
            for &square in line {
                let symbol = if square == Color::None { ' ' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Default for Playground {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_last_line_full_empty() {
        // given
        let playground = Playground::new();

        // when
        let is_last_line_full = playground.is_last_line_full();

        // then
        assert_eq!(is_last_line_full, false);
    }

    #[test]
    fn is_last_line_full_some() {
        // given
        let mut playground = Playground::new();
        playground.set_square(195, Color::Cyan);
        playground.set_square(199, Color::Yellow);

        // when
        let is_last_line_full = playground.is_last_line_full();

        // then
        assert_eq!(is_last_line_full, false);
    }

    #[test]
    fn is_last_line_full_all() {
        // given
        let mut playground = Playground::new();
        (190..200).for_each(|i| playground.set_square(i, Color::Yellow));

        // when
        let is_last_line_full = playground.is_last_line_full();

        // then
        assert_eq!(is_last_line_full, true);
    }

    #[test]
    fn is_cell_bellow_free_true() {
        // given
        let playground = Playground::new();

        // when
        let is_cell_bellow_free = playground.is_cell_bellow_free(56);

        // then
        assert_eq!(is_cell_bellow_free, true);
    }

    #[test]
    fn is_cell_bellow_free_last_line() {
        // given
        let playground = Playground::new();

        // when
        let is_cell_bellow_free = playground.is_cell_bellow_free(196);

        // then
        assert_eq!(is_cell_bellow_free, false);
    }

    #[test]
    fn is_cell_bellow_free_with_color_below() {
        // given
        let mut playground = Playground::new();
        playground.set_square(66, Color::Blue);

        // when
        let is_cell_bellow_free = playground.is_cell_bellow_free(56);

        // then
        assert_eq!(is_cell_bellow_free, false);
    }

    #[test]
    fn is_cell_on_the_right_free_true() {
        // given
        let playground = Playground::new();

        // when
        let is_cell_on_the_right_free = playground.is_cell_on_the_right_free(56);

        // then
        assert_eq!(is_cell_on_the_right_free, true);
    }

    #[test]
    fn is_cell_on_the_right_free_last_column() {
        // given
        let playground = Playground::new();

        // when
        let is_cell_on_the_right_free = playground.is_cell_on_the_right_free(179);

        // then
        assert_eq!(is_cell_on_the_right_free, false);
    }

    #[test]
    fn is_cell_on_the_right_free_color_on_the_right() {
        // given
        let mut playground = Playground::new();
        playground.set_square(66, Color::Blue);

        // when
        let is_cell_on_the_right_free = playground.is_cell_on_the_right_free(65);

        // then
        assert_eq!(is_cell_on_the_right_free, false);
    }

    #[test]
    fn is_cell_on_the_left_free_true() {
        // given
        let playground = Playground::new();

        // when
        let is_cell_on_the_left_free = playground.is_cell_on_the_left_free(56);

        // then
        assert_eq!(is_cell_on_the_left_free, true);
    }

    #[test]
    fn is_cell_on_the_left_free_first_column() {
        // given
        let playground = Playground::new();

        // when
        let is_cell_on_the_left_free = playground.is_cell_on_the_left_free(170);

        // then
        assert_eq!(is_cell_on_the_left_free, false);
    }

    #[test]
    fn is_cell_on_the_left_free_color_on_the_left() {
        // given
        let mut playground = Playground::new();
        playground.set_square(66, Color::Blue);

        // when
        let is_cell_on_the_left_free = playground.is_cell_on_the_left_free(67);

        // then
        assert_eq!(is_cell_on_the_left_free, false);
    }
}
