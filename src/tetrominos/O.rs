use super::{Tetromino, TetrominoPrivate, SQUARE_COUNT};
use crate::color::Color;
use crate::playground::Playground;
use crate::playground::COLUMN_COUNT;
#[allow(unused_imports)]
use rand::{thread_rng, Error, Rng, RngCore};

const COLOR: Color = Color::Yellow;

pub struct O {
    squares: [usize; SQUARE_COUNT],
}

impl O {
    pub fn new(rng: &mut Box<dyn RngCore>, playground: &mut Playground) -> Self {
        let index = rng.gen_range(2, COLUMN_COUNT - 2);
        let mut squares = [0; SQUARE_COUNT];
        squares[0] = index;
        squares[1] = index + 1;
        squares[2] = index + COLUMN_COUNT;
        squares[3] = index + 1 + COLUMN_COUNT;
        playground.set_square(index, COLOR);
        playground.set_square(index + 1, COLOR);
        playground.set_square(index + COLUMN_COUNT, COLOR);
        playground.set_square(index + 1 + COLUMN_COUNT, COLOR);
        O { squares }
    }
}
impl TetrominoPrivate for O {
    fn set_square(&mut self, index: usize, value: usize) {
        self.squares[index] = value;
    }

    fn get_square(&self, index: usize) -> usize {
        self.squares[index]
    }

    fn get_color(&self) -> Color {
        COLOR
    }
}

impl Tetromino for O {
    fn get_color(&self) -> Color {
        COLOR
    }

    fn go_down(&mut self, playground: &mut Playground) -> bool {
        let can_go_down = (0..SQUARE_COUNT).fold(true, |acc, i| {
            let is_cell_bellow_free = playground.is_cell_bellow_free(self.get_square(i));
            let is_cell_bellow_used_by_this_tetromino =
                self.is_cell_used_by_this_tetromino(i, COLUMN_COUNT as i8);
            acc && (is_cell_bellow_free || is_cell_bellow_used_by_this_tetromino)
        });
        if !can_go_down {
            return false;
        }
        self.set_tetromino_on_new_offset(playground, COLUMN_COUNT as i8);
        true
    }

    fn get_squares(&self) -> [usize; SQUARE_COUNT] {
        self.squares
    }

    fn go_right(&mut self, playground: &mut Playground) -> bool {
        let can_go_right = (0..SQUARE_COUNT).fold(true, |acc, i| {
            let is_cell_on_the_right_free =
                playground.is_cell_on_the_right_free(self.get_square(i));
            let is_cell_on_the_right_used_by_this_tetromino =
                self.is_cell_used_by_this_tetromino(i, 1);
            acc && (is_cell_on_the_right_free || is_cell_on_the_right_used_by_this_tetromino)
        });
        if !can_go_right {
            return false;
        }
        self.set_tetromino_on_new_offset(playground, 1);
        true
    }

    fn go_left(&mut self, playground: &mut Playground) -> bool {
        let can_go_left = (0..SQUARE_COUNT).fold(true, |acc, i| {
            let is_cell_on_the_left_free = playground.is_cell_on_the_left_free(self.get_square(i));
            let is_cell_on_the_left_used_by_this_tetromino =
                self.is_cell_used_by_this_tetromino(i, -1);
            acc && (is_cell_on_the_left_free || is_cell_on_the_left_used_by_this_tetromino)
        });
        if !can_go_left {
            return false;
        }
        self.set_tetromino_on_new_offset(playground, -1);
        true
    }

    fn go_down_by(&mut self, length: usize) -> [usize; SQUARE_COUNT] {
        // self.index += COLUMN_COUNT * length;
        self.squares
    }
}

impl Default for O {
    fn default() -> Self {
        let mut rng = Box::new(thread_rng()) as Box<dyn RngCore>;
        Self::new(&mut rng, &mut Playground::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    struct FakeGenerator {
        next_number: u32,
    }
    #[cfg(test)]
    impl RngCore for FakeGenerator {
        fn next_u32(&mut self) -> u32 {
            self.next_number
        }
        fn next_u64(&mut self) -> u64 {
            self.next_number as u64
        }

        fn fill_bytes(&mut self, _dest: &mut [u8]) {}
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), Error> {
            Result::Ok(())
        }
    }
    #[cfg(test)]
    fn get_fake_rand(next_number: u32) -> Box<dyn RngCore> {
        Box::new(FakeGenerator { next_number }) as Box<dyn RngCore>
    }

    #[test]
    fn new() {
        // given
        let mut fake_random = get_fake_rand(2);
        let mut playground = Playground::new();

        // when
        let tetromino = O::new(&mut fake_random, &mut playground);

        // then
        let mut expected_squares = [0; 4];
        expected_squares[0] = 2;
        expected_squares[1] = 3;
        expected_squares[2] = 12;
        expected_squares[3] = 13;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[2], Color::Yellow);
        assert_eq!(playground.get_squares()[3], Color::Yellow);
        assert_eq!(playground.get_squares()[12], Color::Yellow);
        assert_eq!(playground.get_squares()[13], Color::Yellow);
    }

    #[test]
    fn go_down_empty_playground() {
        // given
        let mut fake_random = get_fake_rand(2);
        let mut playground = Playground::new();
        let mut tetromino = O::new(&mut fake_random, &mut playground);

        // when
        let went_down = tetromino.go_down(&mut playground);

        // then
        assert_eq!(went_down, true);
        let mut expected_squares = [0; 4];
        expected_squares[0] = 12;
        expected_squares[1] = 13;
        expected_squares[2] = 22;
        expected_squares[3] = 23;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[2], Color::None);
        assert_eq!(playground.get_squares()[3], Color::None);
        assert_eq!(playground.get_squares()[12], Color::Yellow);
        assert_eq!(playground.get_squares()[13], Color::Yellow);
        assert_eq!(playground.get_squares()[22], Color::Yellow);
        assert_eq!(playground.get_squares()[23], Color::Yellow);
    }

    #[test]
    fn go_down_twice_empty_playground() {
        // given
        let mut fake_random = get_fake_rand(2);
        let mut playground = Playground::new();
        let mut tetromino = O::new(&mut fake_random, &mut playground);

        // when
        let mut went_down = tetromino.go_down(&mut playground);
        assert_eq!(went_down, true);
        went_down = tetromino.go_down(&mut playground);
        assert_eq!(went_down, true);

        // then
        let mut expected_squares = [0; 4];
        expected_squares[0] = 22;
        expected_squares[1] = 23;
        expected_squares[2] = 32;
        expected_squares[3] = 33;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[2], Color::None);
        assert_eq!(playground.get_squares()[3], Color::None);
        assert_eq!(playground.get_squares()[12], Color::None);
        assert_eq!(playground.get_squares()[13], Color::None);
        assert_eq!(playground.get_squares()[22], Color::Yellow);
        assert_eq!(playground.get_squares()[23], Color::Yellow);
        assert_eq!(playground.get_squares()[32], Color::Yellow);
        assert_eq!(playground.get_squares()[33], Color::Yellow);
    }

    #[test]
    fn go_down_blocked_by_square() {
        // given
        let mut fake_random = get_fake_rand(2);
        let mut playground = Playground::new();
        playground.set_square(22, Color::Cyan);
        let mut tetromino = O::new(&mut fake_random, &mut playground);

        // when
        let went_down = tetromino.go_down(&mut playground);

        // then
        assert_eq!(went_down, false);
        let mut expected_squares = [0; 4];
        expected_squares[0] = 2;
        expected_squares[1] = 3;
        expected_squares[2] = 12;
        expected_squares[3] = 13;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[2], Color::Yellow);
        assert_eq!(playground.get_squares()[3], Color::Yellow);
        assert_eq!(playground.get_squares()[12], Color::Yellow);
        assert_eq!(playground.get_squares()[13], Color::Yellow);
        assert_eq!(playground.get_squares()[22], Color::Cyan);
        assert_eq!(playground.get_squares()[23], Color::None);
    }

    #[test]
    fn go_left_empty_playground() {
        // given
        let mut fake_random = get_fake_rand(2);
        let mut playground = Playground::new();
        let mut tetromino = O::new(&mut fake_random, &mut playground);

        // when
        let went_left = tetromino.go_left(&mut playground);

        // then
        assert_eq!(went_left, true);
        let mut expected_squares = [0; 4];
        expected_squares[0] = 1;
        expected_squares[1] = 2;
        expected_squares[2] = 11;
        expected_squares[3] = 12;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[3], Color::None);
        assert_eq!(playground.get_squares()[13], Color::None);
        assert_eq!(playground.get_squares()[1], Color::Yellow);
        assert_eq!(playground.get_squares()[2], Color::Yellow);
        assert_eq!(playground.get_squares()[11], Color::Yellow);
        assert_eq!(playground.get_squares()[12], Color::Yellow);
    }

    #[test]
    fn go_left_twice_empty_playground() {
        // given
        let mut fake_random = get_fake_rand(2);
        let mut playground = Playground::new();
        let mut tetromino = O::new(&mut fake_random, &mut playground);

        // when
        let mut went_left = tetromino.go_left(&mut playground);
        assert_eq!(went_left, true);
        went_left = tetromino.go_left(&mut playground);
        assert_eq!(went_left, true);

        // then
        let mut expected_squares = [0; 4];
        expected_squares[0] = 0;
        expected_squares[1] = 1;
        expected_squares[2] = 10;
        expected_squares[3] = 11;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[2], Color::None);
        assert_eq!(playground.get_squares()[3], Color::None);
        assert_eq!(playground.get_squares()[12], Color::None);
        assert_eq!(playground.get_squares()[13], Color::None);
        assert_eq!(playground.get_squares()[0], Color::Yellow);
        assert_eq!(playground.get_squares()[1], Color::Yellow);
        assert_eq!(playground.get_squares()[10], Color::Yellow);
        assert_eq!(playground.get_squares()[11], Color::Yellow);
    }

    #[test]
    fn go_left_blocked_by_square() {
        // given
        let mut fake_random = get_fake_rand(2);
        let mut playground = Playground::new();
        playground.set_square(1, Color::Cyan);
        let mut tetromino = O::new(&mut fake_random, &mut playground);

        // when
        let went_left = tetromino.go_left(&mut playground);

        // then
        assert_eq!(went_left, false);
        let mut expected_squares = [0; 4];
        expected_squares[0] = 2;
        expected_squares[1] = 3;
        expected_squares[2] = 12;
        expected_squares[3] = 13;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[2], Color::Yellow);
        assert_eq!(playground.get_squares()[3], Color::Yellow);
        assert_eq!(playground.get_squares()[12], Color::Yellow);
        assert_eq!(playground.get_squares()[13], Color::Yellow);
        assert_eq!(playground.get_squares()[1], Color::Cyan);
        assert_eq!(playground.get_squares()[11], Color::None);
    }

    #[test]
    fn go_right_empty_playground() {
        // given
        let mut fake_random = get_fake_rand(2);
        let mut playground = Playground::new();
        let mut tetromino = O::new(&mut fake_random, &mut playground);

        // when
        let went_right = tetromino.go_right(&mut playground);

        // then
        assert_eq!(went_right, true);
        let mut expected_squares = [0; 4];
        expected_squares[0] = 3;
        expected_squares[1] = 4;
        expected_squares[2] = 13;
        expected_squares[3] = 14;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[2], Color::None);
        assert_eq!(playground.get_squares()[12], Color::None);
        assert_eq!(playground.get_squares()[3], Color::Yellow);
        assert_eq!(playground.get_squares()[4], Color::Yellow);
        assert_eq!(playground.get_squares()[13], Color::Yellow);
        assert_eq!(playground.get_squares()[14], Color::Yellow);
    }

    #[test]
    fn go_right_twice_empty_playground() {
        // given
        let mut fake_random = get_fake_rand(2);
        let mut playground = Playground::new();
        let mut tetromino = O::new(&mut fake_random, &mut playground);

        // when
        let mut went_right = tetromino.go_right(&mut playground);
        assert_eq!(went_right, true);
        went_right = tetromino.go_right(&mut playground);
        assert_eq!(went_right, true);

        // then
        let mut expected_squares = [0; 4];
        expected_squares[0] = 4;
        expected_squares[1] = 5;
        expected_squares[2] = 14;
        expected_squares[3] = 15;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[2], Color::None);
        assert_eq!(playground.get_squares()[3], Color::None);
        assert_eq!(playground.get_squares()[12], Color::None);
        assert_eq!(playground.get_squares()[13], Color::None);
        assert_eq!(playground.get_squares()[4], Color::Yellow);
        assert_eq!(playground.get_squares()[5], Color::Yellow);
        assert_eq!(playground.get_squares()[14], Color::Yellow);
        assert_eq!(playground.get_squares()[15], Color::Yellow);
    }

    #[test]
    fn go_right_blocked_by_square() {
        // given
        let mut fake_random = get_fake_rand(2);
        let mut playground = Playground::new();
        playground.set_square(4, Color::Cyan);
        let mut tetromino = O::new(&mut fake_random, &mut playground);

        // when
        let went_right = tetromino.go_right(&mut playground);

        // then
        assert_eq!(went_right, false);
        let mut expected_squares = [0; 4];
        expected_squares[0] = 2;
        expected_squares[1] = 3;
        expected_squares[2] = 12;
        expected_squares[3] = 13;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[2], Color::Yellow);
        assert_eq!(playground.get_squares()[3], Color::Yellow);
        assert_eq!(playground.get_squares()[12], Color::Yellow);
        assert_eq!(playground.get_squares()[13], Color::Yellow);
        assert_eq!(playground.get_squares()[4], Color::Cyan);
        assert_eq!(playground.get_squares()[14], Color::None);
    }
}
