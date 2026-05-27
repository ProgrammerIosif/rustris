use crate::{board::*, piece::Piece};

const LEVEL_SPEED: [u32; 30] = [
    48, 43, 38, 33, 28, 23, 18, 13, 8, 6, 5, 5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    1,
];

pub enum Action {
    MoveLeft,
    MoveDown,
    MoveRight,
    RotateClockWise,
    RotateCounterClockWise,
}

pub struct Game {
    board: Board,
    pub piece: Piece,
    pub next_piece: Piece,
    level: u32,
    lines: u32,
    score: u32,
    frame: u32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            piece: Piece::new(),
            next_piece: Piece::new(),
            level: 0,
            lines: 0,
            score: 0,
            frame: 0,
        }
    }

    pub fn is_filled(&self, x: usize, y: usize) -> bool {
        self.board.is_filled(x, y) || self.piece.occupies(x, y)
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn lines(&self) -> u32 {
        self.lines
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    fn update_stats(&mut self) {
        let lines_cleared = self.board.clear_lines();
        let multiplier = match lines_cleared {
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => 0,
        };
        self.score += multiplier * (self.level + 1);
        self.lines += lines_cleared;
        self.level = self.lines / 10;
    }

    pub fn increment_frame(&mut self) {
        self.frame += 1;
        if self.frame >= LEVEL_SPEED[self.level as usize] {
            self.frame = 0;
            self.update(Action::MoveDown);
        }
    }

    pub fn update(&mut self, action: Action) {
        let old_piece = self.piece;
        match action {
            Action::MoveLeft => self.piece.move_left(),
            Action::MoveRight => self.piece.move_right(),
            Action::RotateCounterClockWise => self.piece.rotate_counterclockwise(),
            Action::RotateClockWise => self.piece.rotate_clockwise(),
            Action::MoveDown => self.piece.move_down(),
        }
        if self.board.collision(&self.piece) {
            self.piece = old_piece;
            if let Action::MoveDown = action {
                self.board.add_piece_to_stack(&self.piece);
                self.update_stats();
                self.piece = self.next_piece;
                self.next_piece = Piece::new();
            }
        }
    }
}
