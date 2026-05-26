mod board;
mod game;
mod piece;
mod render;
mod tetrominoes;

use crate::game::Game;

fn main() {
    let mut game = Game::new();
    render::render(&game);
    let mut input = String::new();
    loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        game.update(input.chars().next().expect(""));
        render::render(&game);
    }
}
