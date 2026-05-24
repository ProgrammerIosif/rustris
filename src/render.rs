use crate::{
    board::*,
    piece::{Piece, Position},
};

pub fn render(board: &Board, piece: &Piece) {
    let board = board.get_data();
    // for square_position in piece.get_squares() {
    //     let Position { x, y } = square_position + piece.get_position();
    //     if x >= 0 && y >= 0 {
    //         to_render[y as usize][x as usize] = Some(1);
    //     }
    // }
    let pos = piece.get_position();
    let squares = piece.get_squares();
    let piece_data: [Position; 4] = std::array::from_fn(|i| squares[i] + pos);

    for i in 0..BOARD_HEIGHT {
        for j in 0..BOARD_WIDTH {
            if board[i][j].is_some() || piece_data.contains(&Position{x: j as i8,y: i as i8}) {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();
    }
}
