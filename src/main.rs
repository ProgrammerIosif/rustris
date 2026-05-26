mod board;
mod piece;
mod render;
mod tetrominoes;

use crate::{board::Board, piece::Piece};

fn update_stats(level: &mut u32, lines: &mut u32, score: &mut u32, lines_cleared: u32) {
    let multiplier = match lines_cleared {
        1 => 40,
        2 => 100,
        3 => 300,
        4 => 1200,
        _ => 0,
    };
    *score += multiplier * (*level + 1);
    *lines += lines_cleared;
    *level = *lines / 10;
}

fn main() {
    // testing and prototyping
    let mut board = Board::new();
    let mut piece = Piece::new();
    let mut next_piece = Piece::new();
    let mut lines = 0;
    let mut level = 0;
    let mut score = 0;
    render::render(&board, &piece, &next_piece, &level, &lines, &score);
    let mut key;
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        key = input.chars().next().expect("");
        let old_piece = piece;
        match key {
            'a' => piece.move_left(),
            's' => piece.move_down(),
            'd' => piece.move_right(),
            'z' => piece.rotate_counterclockwise(),
            'x' => piece.rotate_clockwise(),
            _ => (),
        }
        if board.collision(&piece) {
            piece = old_piece;
            if key == 's' {
                board.add_piece_to_stack(&piece);
                update_stats(&mut level, &mut lines, &mut score, board.clear_lines());
                piece = next_piece;
                next_piece = Piece::new();
            }
        }
        render::render(&board, &piece, &next_piece, &level, &lines, &score);
    }

    // game loop:
    // 1. display
    // 2. input
    // 3. gravity
    // 4. check collisions
}
