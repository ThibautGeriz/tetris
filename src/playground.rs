use crate::color::Color;
use std::fmt;

pub const COLUMN_COUNT: usize = 10;
pub const ROW_COUNT: usize = 20;
pub const SQUARE_COUNT: usize = COLUMN_COUNT * ROW_COUNT;

#[derive(Clone)]
pub struct Playground(Vec<Color>);

impl Playground {
    pub fn new() -> Playground {
        let squares = (0..COLUMN_COUNT * ROW_COUNT)
            .map(|_i| Color::None)
            .collect();
        Playground(squares)
    }

    pub fn remove_full_lines_at_the_end(&mut self) -> u8 {
        let mut full_square_count = 0;
        for index in (0..SQUARE_COUNT).rev() {
            if self.0[index] == Color::None {
                break;
            }
            full_square_count += 1;
        }
        let line_removed_count = full_square_count / COLUMN_COUNT as u8;
        (0..(COLUMN_COUNT * line_removed_count as usize)).for_each(|_| {
            self.0.insert(0, Color::None);
            self.0.remove(self.0.len() - 1);
        });
        line_removed_count
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

    #[cfg(test)]
    impl Playground {
        #[cfg(test)]
        pub fn is_empty(&self) -> bool {
            self.0
                .as_slice()
                .iter()
                .all(|square| *square == Color::None)
        }
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

    #[test]
    fn remove_full_lines_at_the_end_empty_playground() {
        // given
        let mut playground = Playground::new();

        // when
        let number_of_mine_removed = playground.remove_full_lines_at_the_end();

        // then
        assert_eq!(number_of_mine_removed, 0);
        assert!(playground.is_empty());
    }

    #[test]
    fn remove_full_lines_at_the_end_full_playground() {
        // given
        let squares = (0..COLUMN_COUNT * ROW_COUNT)
            .map(|_i| Color::Cyan)
            .collect();
        let mut playground = Playground(squares);

        // when
        let number_of_mine_removed = playground.remove_full_lines_at_the_end();

        // then
        assert_eq!(number_of_mine_removed, 20);
        assert!(playground.is_empty());
    }

    #[test]
    fn remove_full_lines_at_the_end_with_two_and_half_line_full() {
        // given
        let squares = (0..COLUMN_COUNT * ROW_COUNT)
            .map(|i| if i < 175 { Color::None } else { Color::Cyan })
            .collect();
        let mut playground = Playground(squares);

        // when
        let number_of_mine_removed = playground.remove_full_lines_at_the_end();

        // then
        assert_eq!(number_of_mine_removed, 2);
        playground
            .get_squares()
            .as_slice()
            .iter()
            .enumerate()
            .for_each(|(i, square)| {
                if i < 195 {
                    assert_eq!(*square, Color::None);
                } else {
                    assert_eq!(*square, Color::Cyan);
                }
            })
    }
}
