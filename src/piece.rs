use crate::tetrominoes::{Tetromino, get_orientations};
use std::ops::Add;

/// Coordinate system convention:
/// x - column(left to right)
/// y - row(top to bottom)
/// (0,0) - top-left corner
#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub const SPAWN_POINT: Position = Position { x: 5, y: 0 };

/// position: the position of the "center of the piece" on the grid
/// orientations: the positions of the squares relative to the piece position
/// orientation_index: the current configuration of the piece
pub struct Piece {
    position: Position,
    orientation_index: usize,
    orientations: &'static [[Position; 4]],
}

impl Piece {
    pub fn new(tetromino: Tetromino) -> Self {
        Piece {
            position: SPAWN_POINT,
            orientation_index: 0,
            orientations: get_orientations(&tetromino),
        }
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn get_squares(&self) -> [Position; 4] {
        self.orientations[self.orientation_index]
    }

    pub fn rotate_clockwise(&mut self) {
        self.orientation_index = (self.orientation_index + 1) % self.orientations.len();
    }

    pub fn rotate_counterclockwise(&mut self) {
        self.orientation_index =
            (self.orientation_index + self.orientations.len() - 1) % self.orientations.len()
    }

    pub fn move_down(&mut self) {
        self.position.y += 1;
    }

    pub fn move_left(&mut self) {
        self.position.x -= 1;
    }

    pub fn move_right(&mut self) {
        self.position.x += 1;
    }
}
