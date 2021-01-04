use super::{Tetromino, TetrominoCommon, SQUARE_COUNT};
use crate::color::Color;
use crate::playground::Playground;
use crate::playground::COLUMN_COUNT;
#[allow(unused_imports)]
use rand::{thread_rng, Error, Rng, RngCore};

const COLOR: Color = Color::Purple;

pub struct T {
    squares: [usize; SQUARE_COUNT],
}

impl T {
    fn create(rng: &mut Box<dyn RngCore>) -> Self {
        let index = rng.gen_range(2, COLUMN_COUNT - 2);
        let mut squares = [0; SQUARE_COUNT];
        squares[0] = index;
        squares[1] = index + 1;
        squares[2] = index + 2;
        squares[3] = index + COLUMN_COUNT + 1;
        T { squares }
    }
}
impl TetrominoCommon for T {
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

impl Tetromino for T {
    fn new() -> Self {
        let mut rng = Box::new(thread_rng()) as Box<dyn RngCore>;
        T::create(&mut rng)
    }

    fn insert_into_playground(&self, playground: &mut Playground) -> bool {
        <T as TetrominoCommon>::insert_into_playground(self, playground)
    }

    fn go_down(&mut self, playground: &mut Playground) -> bool {
        <T as TetrominoCommon>::go_down(self, playground)
    }

    fn go_right(&mut self, playground: &mut Playground) -> bool {
        <T as TetrominoCommon>::go_right(self, playground)
    }

    fn go_left(&mut self, playground: &mut Playground) -> bool {
        <T as TetrominoCommon>::go_left(self, playground)
    }

    fn go_bottom(&mut self, playground: &mut Playground) -> bool {
        <T as TetrominoCommon>::go_bottom(self, playground)
    }
}

impl Default for T {
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
        let tetromino = T::create(&mut fake_random);

        // when
        <T as TetrominoCommon>::insert_into_playground(&tetromino, &mut playground);

        // then
        let mut expected_squares = [0; 4];
        expected_squares[0] = 2;
        expected_squares[1] = 3;
        expected_squares[2] = 4;
        expected_squares[3] = 13;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[2], Color::Purple);
        assert_eq!(playground.get_squares()[3], Color::Purple);
        assert_eq!(playground.get_squares()[4], Color::Purple);
        assert_eq!(playground.get_squares()[13], Color::Purple);
    }

    #[test]
    fn go_down_empty_playground() {
        // given
        let mut fake_random = get_fake_rand(2);
        let mut playground = Playground::new();
        let mut tetromino = T::create(&mut fake_random);
        <T as TetrominoCommon>::insert_into_playground(&tetromino, &mut playground);

        // when
        let went_down = <T as Tetromino>::go_down(&mut tetromino, &mut playground);

        // then
        assert_eq!(went_down, true);
        let mut expected_squares = [0; 4];
        expected_squares[0] = 12;
        expected_squares[1] = 13;
        expected_squares[2] = 14;
        expected_squares[3] = 23;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[2], Color::None);
        assert_eq!(playground.get_squares()[3], Color::None);
        assert_eq!(playground.get_squares()[4], Color::None);
        assert_eq!(playground.get_squares()[12], Color::Purple);
        assert_eq!(playground.get_squares()[13], Color::Purple);
        assert_eq!(playground.get_squares()[14], Color::Purple);
        assert_eq!(playground.get_squares()[23], Color::Purple);
    }
}
