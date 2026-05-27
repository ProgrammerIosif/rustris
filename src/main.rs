mod app;
mod board;
mod game;
mod piece;
mod tetrominoes;

use crate::app::App;

fn main() -> eframe::Result {
    eframe::run_native(
        "Tetris",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}
