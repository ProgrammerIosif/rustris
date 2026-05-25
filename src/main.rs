mod board;
mod piece;
mod render;
mod tetrominoes;

use crate::{board::Board, piece::Piece};

fn main() {
    // testing and prototyping
    let mut board = Board::new();
    let mut piece = Piece::new();
    let mut next_piece = Piece::new();
    render::render(&board, &piece, &next_piece);
    let mut key;
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        key = input.chars().next().expect("");
        match key {
            'a' => piece.move_left(),
            's' => piece.move_down(),
            'd' => piece.move_right(),
            'z' => piece.rotate_counterclockwise(),
            'x' => piece.rotate_clockwise(),
            _ => (),
        }
        if board.collision(&piece) {
            match key {
                'd' => piece.move_left(),
                's' => {
                    piece.move_up();
                    board.add_piece_to_stack(&piece);
                    board.clear_lines();
                    piece = next_piece;
                    next_piece = Piece::new();
                }
                'a' => piece.move_right(),
                'x' => piece.rotate_counterclockwise(),
                'z' => piece.rotate_clockwise(),
                _ => (),
            }
        }
        render::render(&board, &piece, &next_piece);
    }

    // game loop:
    // 1. display
    // 2. input
    // 3. gravity
    // 4. check collisions
}
