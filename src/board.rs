use crate::piece::Piece;
use crate::piece::Position;

/// Square can be:
/// -None, the cell is empty
/// -Some(x), the cell is occupied, with a color x
pub type Square = Option<i8>;

pub const BOARD_HEIGHT: usize = 10;
pub const BOARD_WIDTH: usize = 10;

pub struct Board([[Square; BOARD_WIDTH]; BOARD_HEIGHT]);

impl Board {
    pub fn new() -> Self {
        Self([[None; BOARD_WIDTH]; BOARD_HEIGHT])
    }

    pub fn get_data(&self) -> &[[Square; BOARD_WIDTH]; BOARD_HEIGHT] {
        &self.0
    }

    pub fn collision(&self, piece: &Piece) -> bool {
        let piece_position = piece.get_position();
        for relative_position in piece.get_squares() {
            let Position { x, y } = piece_position + relative_position;
            // out of board bounds
            if !(0..BOARD_WIDTH as i8).contains(&x) || y >= BOARD_HEIGHT as i8 {
                return true;
            }
            // touches the stack
            if y >= 0 && self.0[y as usize][x as usize].is_some() {
                return true;
            }
        }
        false
    }

    pub fn add_piece_to_stack(&mut self, piece: &Piece) {
        let piece_position = piece.get_position();
        for relative_position in piece.get_squares() {
            let Position { x, y } = piece_position + relative_position;
            self.0[y as usize][x as usize] = Some(1);
        }
    }

    pub fn clear_lines(&mut self) -> usize {
        let mut count = 0;
        for i in (0..10).rev() {
            if self.0[i].iter().all(|x| x.is_some()) {
                count += 1;
            } else if count != 0 {
                self.0[i + count] = self.0[i];
            }
        }
        for i in 0..count {
            self.0[i] = [None; 10];
        }
        count
    }
}
