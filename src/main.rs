mod board;
mod piece;
mod render;
mod tetrominoes;

use crate::{board::Board, piece::Piece, tetrominoes::Tetromino};

fn main() {
    // testing and prototyping
    let board = Board::new();
    let mut piece = Piece::new(Tetromino::O);
    let squares = piece.get_squares();
    squares.iter().for_each(|c| println!("{},{}", c.x, c.y));
    render::render(&board, &piece);
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.chars().next().expect("") {
            'a' => piece.move_left(),
            's' => piece.move_down(),
            'd' => piece.move_right(),
            'z' => piece.rotate_counterclockwise(),
            'x' => piece.rotate_clockwise(),
            _ => (),
        }
        if board.collision(&piece) {
            match input.chars().next().expect("") {
                'd' => piece.move_left(),
                'a' => piece.move_right(),
                'x' => piece.rotate_counterclockwise(),
                'z' => piece.rotate_clockwise(),
                _ => (),
            }
        }

        render::render(&board, &piece);
    }

    // game loop:
    // 1. display
    // 2. input
    // 3. gravity
    // 4. check collisions
}
