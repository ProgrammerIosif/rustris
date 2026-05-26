use crate::{board::*, piece::Piece};

pub struct Game {
    board: Board,
    pub piece: Piece,
    pub next_piece: Piece,
    level: u32,
    lines: u32,
    score: u32,
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
        }
    }

    pub fn board(&self) -> &[[Square; BOARD_WIDTH]; BOARD_HEIGHT] {
        self.board.get_data()
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

    pub fn update(&mut self, key: char) {
        let old_piece = self.piece;
        match key {
            'a' => self.piece.move_left(),
            's' => self.piece.move_down(),
            'd' => self.piece.move_right(),
            'z' => self.piece.rotate_counterclockwise(),
            'x' => self.piece.rotate_clockwise(),
            _ => (),
        }
        if self.board.collision(&self.piece) {
            self.piece = old_piece;
            if key == 's' {
                self.board.add_piece_to_stack(&self.piece);
                self.update_stats();
                self.piece = self.next_piece;
                self.next_piece = Piece::new();
            }
        }
    }
}
