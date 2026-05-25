use crate::{
    board::*,
    piece::{Piece, Position},
};

pub fn render(board: &Board, piece: &Piece, next_piece: &Piece) {
    let board = board.get_data();

    let pos = piece.get_position();
    let squares = piece.get_squares();
    let piece_data: [Position; 4] = std::array::from_fn(|i| squares[i] + pos);

    let next_piece_data = next_piece.get_squares();

    for i in 0..BOARD_HEIGHT {
        for j in 0..BOARD_WIDTH {
            if board[i][j].is_some()
                || piece_data.contains(&Position {
                    x: j as i8,
                    y: i as i8,
                })
            {
                print!("#");
            } else {
                print!(".");
            }
        }

        if i == 1 || i == 2 {
            print!("  ");
            for j in -2..2 {
                if next_piece_data.contains(&Position {x: j, y: -1 + i as i8}) {
                    print!("#");
                }
                else {
                    print!(" ");
                }
            }
        }

        println!();
    }
}
