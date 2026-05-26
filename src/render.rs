use crate::{board::*, game::Game, piece::Position};

pub fn render(game: &Game) {
    let board = game.board();
    let pos = game.piece.get_position();
    let squares = game.piece.get_squares();
    let piece_data: [Position; 4] = std::array::from_fn(|i| squares[i] + pos);
    let next_piece_data = game.next_piece.get_squares();

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
            1 => print!("  Level {}", game.level()),
            2 => print!("  Lines {}", game.lines()),
            3 => print!("  Score {}", game.score()),
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
