use crate::{
    board::*,
    game::{Action, Game},
};

pub struct App {
    game: Game,
}

impl App {
    pub fn new() -> Self {
        Self { game: Game::new() }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let action = get_input(ui);

        if let Some(a) = action {
            self.game.update(a);
        }
        self.game.increment_frame();

        egui::CentralPanel::default().show_inside(ui, |ui| {
            render(&self.game, ui);
        });

        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(16));
    }
}

fn get_input(ui: &egui::Ui) -> Option<Action> {
    ui.input(|i| {
        if i.key_pressed(egui::Key::ArrowLeft) {
            Some(Action::MoveLeft)
        } else if i.key_pressed(egui::Key::ArrowRight) {
            Some(Action::MoveRight)
        } else if i.key_pressed(egui::Key::ArrowDown) {
            Some(Action::MoveDown)
        } else if i.key_pressed(egui::Key::Z) {
            Some(Action::RotateCounterClockWise)
        } else if i.key_pressed(egui::Key::X) {
            Some(Action::RotateClockWise)
        } else {
            None
        }
    })
}

fn render(game: &Game, ui: &mut egui::Ui) {
    let cell_size = 20.0;

    ui.horizontal(|ui| {
        // BOARD
        {
            let painter = ui.painter();

            let origin = ui.cursor().min;

            for y in 0..BOARD_HEIGHT {
                for x in 0..BOARD_WIDTH {
                    let rect = egui::Rect::from_min_size(
                        egui::pos2(
                            origin.x + x as f32 * cell_size,
                            origin.y + y as f32 * cell_size,
                        ),
                        egui::vec2(cell_size, cell_size),
                    );

                    let filled = game.is_filled(x, y);

                    painter.rect_filled(
                        rect,
                        0,
                        if filled {
                            egui::Color32::WHITE
                        } else {
                            egui::Color32::DARK_GRAY
                        },
                    );
                }
            }

            // Reserve space so layouts work correctly
            ui.allocate_space(egui::vec2(
                BOARD_WIDTH as f32 * cell_size,
                BOARD_HEIGHT as f32 * cell_size,
            ));
        }
        ui.add_space(20.0);

        // SIDE PANEL
        ui.vertical(|ui| {
            ui.heading("Stats");

            ui.separator();

            ui.label(format!("Level: {}", game.level()));
            ui.label(format!("Lines: {}", game.lines()));
            ui.label(format!("Score: {}", game.score()));

            ui.add_space(20.0);

            ui.heading("Next");
            let preview_size = 4.0 * cell_size;
            let (rect, _) = ui
                .allocate_exact_size(egui::vec2(preview_size, preview_size), egui::Sense::hover());

            let painter = ui.painter();
            painter.rect_filled(rect, 2.0, egui::Color32::from_gray(30));

            let preview_origin = rect.min;
            for square in game.next_piece.get_squares() {
                let x = square.x as f32 + 2.0;
                let y = square.y as f32 + 2.0;
                let square_rect = egui::Rect::from_min_size(
                    egui::pos2(
                        preview_origin.x + x * cell_size,
                        preview_origin.y + y * cell_size,
                    ),
                    egui::vec2(cell_size, cell_size),
                );
                painter.rect_filled(square_rect, 0, egui::Color32::LIGHT_BLUE);
            }
        });
    });
}
