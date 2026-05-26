use crate::{
    board::*,
    piece::{Piece, Position},
};

pub fn render(
    board: &Board,
    piece: &Piece,
    next_piece: &Piece,
    level: &u32,
    lines: &u32,
    score: &u32,
) {
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

        match i {
            1 => print!("  Level {}", level),
            2 => print!("  Lines {}", lines),
            3 => print!("  Score {}", score),
            4 | 5 => {
                print!("  ");
                for j in -2..2 {
                    if next_piece_data.contains(&Position {
                        x: j,
                        y: -4 + i as i8,
                    }) {
                        print!("#");
                    } else {
                        print!(" ");
                    }
                }
            }
            _ => (),
        };

        println!();
    }
}
