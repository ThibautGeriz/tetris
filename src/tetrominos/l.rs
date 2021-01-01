use super::{Tetromino, TetrominoCommon, SQUARE_COUNT};
use crate::color::Color;
use crate::playground::Playground;
use crate::playground::COLUMN_COUNT;
#[allow(unused_imports)]
use rand::{thread_rng, Error, Rng, RngCore};

const COLOR: Color = Color::Orange;

pub struct L {
    squares: [usize; SQUARE_COUNT],
}

impl L {
    pub fn new(playground: &mut Playground) -> Self {
        let mut rng = Box::new(thread_rng()) as Box<dyn RngCore>;
        L::create(&mut rng, playground)
    }

    fn create(rng: &mut Box<dyn RngCore>, playground: &mut Playground) -> Self {
        let index = rng.gen_range(2, COLUMN_COUNT - 2);
        let mut squares = [0; SQUARE_COUNT];
        squares[0] = index;
        squares[1] = index + COLUMN_COUNT;
        squares[2] = index + 2 * COLUMN_COUNT;
        squares[3] = index + 2 * COLUMN_COUNT + 1;
        (0..SQUARE_COUNT).for_each(|i| {
            playground.set_square(squares[i], COLOR);
        });
        L { squares }
    }
}
impl TetrominoCommon for L {
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

impl Tetromino for L {
    fn go_down(&mut self, playground: &mut Playground) -> bool {
        <L as TetrominoCommon>::go_down(self, playground)
    }

    fn go_right(&mut self, playground: &mut Playground) -> bool {
        <L as TetrominoCommon>::go_right(self, playground)
    }

    fn go_left(&mut self, playground: &mut Playground) -> bool {
        <L as TetrominoCommon>::go_left(self, playground)
    }

    fn go_bottom(&mut self, playground: &mut Playground) -> bool {
        <L as TetrominoCommon>::go_bottom(self, playground)
    }
}

impl Default for L {
    fn default() -> Self {
        Self::new(&mut Playground::default())
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
        let tetromino = L::create(&mut fake_random, &mut playground);

        // then
        let mut expected_squares = [0; 4];
        expected_squares[0] = 2;
        expected_squares[1] = 12;
        expected_squares[2] = 22;
        expected_squares[3] = 23;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[2], Color::Orange);
        assert_eq!(playground.get_squares()[12], Color::Orange);
        assert_eq!(playground.get_squares()[22], Color::Orange);
        assert_eq!(playground.get_squares()[23], Color::Orange);
    }

    #[test]
    fn go_down_empty_playground() {
        // given
        let mut fake_random = get_fake_rand(2);
        let mut playground = Playground::new();
        let mut tetromino = L::create(&mut fake_random, &mut playground);

        // when
        let went_down = <L as Tetromino>::go_down(&mut tetromino, &mut playground);

        // then
        assert_eq!(went_down, true);
        let mut expected_squares = [0; 4];
        expected_squares[0] = 12;
        expected_squares[1] = 22;
        expected_squares[2] = 32;
        expected_squares[3] = 33;
        assert_eq!(tetromino.squares, expected_squares);
        assert_eq!(playground.get_squares()[2], Color::None);
        assert_eq!(playground.get_squares()[23], Color::None);
        assert_eq!(playground.get_squares()[12], Color::Orange);
        assert_eq!(playground.get_squares()[22], Color::Orange);
        assert_eq!(playground.get_squares()[32], Color::Orange);
        assert_eq!(playground.get_squares()[33], Color::Orange);
    }
}
