use super::{Tetromino, TetrominoCommon, SQUARE_COUNT};
use crate::color::Color;
use crate::playground::Playground;
use crate::playground::COLUMN_COUNT;
#[allow(unused_imports)]
use rand::{thread_rng, Error, Rng, RngCore};

const COLOR: Color = Color::Green;

pub struct S {
    squares: [usize; SQUARE_COUNT],
}

impl S {
    fn create(rng: &mut Box<dyn RngCore>) -> Self {
        let index = rng.gen_range(2, COLUMN_COUNT - 2);
        let mut squares = [0; SQUARE_COUNT];
        squares[0] = index + 1;
        squares[1] = index + 2;
        squares[2] = index + COLUMN_COUNT;
        squares[3] = index + COLUMN_COUNT + 1;
        S { squares }
    }
}
impl TetrominoCommon for S {
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

impl Tetromino for S {
    fn new() -> Self {
        let mut rng = Box::new(thread_rng()) as Box<dyn RngCore>;
        S::create(&mut rng)
    }

    fn insert_into_playground(&self, playground: &mut Playground) -> bool {
        <S as TetrominoCommon>::insert_into_playground(self, playground)
    }

    fn go_down(&mut self, playground: &mut Playground) -> bool {
        <S as TetrominoCommon>::go_down(self, playground)
    }

    fn go_right(&mut self, playground: &mut Playground) -> bool {
        <S as TetrominoCommon>::go_right(self, playground)
    }

    fn go_left(&mut self, playground: &mut Playground) -> bool {
        <S as TetrominoCommon>::go_left(self, playground)
    }

    fn go_bottom(&mut self, playground: &mut Playground) -> bool {
        <S as TetrominoCommon>::go_bottom(self, playground)
    }

    fn rotate_clockwise(&mut self, playground: &mut Playground) -> bool {
        true
    }

    fn rotate_anticlockwise(&mut self, playground: &mut Playground) -> bool {
        true
    }
}

impl Default for S {
    fn default() -> Self {
        Self::new()
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
        let tetromino = S::create(&mut fake_random);

        // when
        <S as TetrominoCommon>::insert_into_playground(&tetromino, &mut playground);

        // then
        let mut expected_squares = [0; 4];
        expected_squares[0] = 3;
        expected_squares[1] = 4;
        expected_squares[2] = 12;
        expected_squares[3] = 13;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[3], Color::Green);
        assert_eq!(playground.get_squares()[4], Color::Green);
        assert_eq!(playground.get_squares()[12], Color::Green);
        assert_eq!(playground.get_squares()[13], Color::Green);
    }

    #[test]
    fn go_down_empty_playground() {
        // given
        let mut fake_random = get_fake_rand(2);
        let mut playground = Playground::new();
        let mut tetromino = S::create(&mut fake_random);
        <S as TetrominoCommon>::insert_into_playground(&tetromino, &mut playground);

        // when
        let went_down = <S as Tetromino>::go_down(&mut tetromino, &mut playground);

        // then
        assert_eq!(went_down, true);
        let mut expected_squares = [0; 4];
        expected_squares[0] = 13;
        expected_squares[1] = 14;
        expected_squares[2] = 22;
        expected_squares[3] = 23;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[3], Color::None);
        assert_eq!(playground.get_squares()[4], Color::None);
        assert_eq!(playground.get_squares()[13], Color::Green);
        assert_eq!(playground.get_squares()[14], Color::Green);
        assert_eq!(playground.get_squares()[22], Color::Green);
        assert_eq!(playground.get_squares()[23], Color::Green);
    }
}
