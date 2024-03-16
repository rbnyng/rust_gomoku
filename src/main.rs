// Disable terminal in Windows compiled executable
#![windows_subsystem = "windows"]

use eframe::egui::{self};
use serde::{Deserialize, Serialize};

const BOARD_SIZE: usize = 15;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Player {
    Black,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum GameState {
    Ongoing,
    WonBy(Player),
    Draw,
}

#[derive(Serialize, Deserialize)]
struct GomokuApp {
    board: [[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
    current_player: Player,
    history: Vec<(
        [[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
        Player,
        (usize, usize),
    )>,
    show_help: bool,
    game_state: GameState,
}

impl Default for GomokuApp {
    fn default() -> Self {
        Self {
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            current_player: Player::Black,
            history: Vec::new(),
            show_help: false,
            game_state: GameState::Ongoing,
        }
    }
}

/// Converts a column index to a corresponding letter (A, B, C, ...).
/// Assumes 0-based indexing for columns.
fn column_index_to_letter(index: usize) -> char {
    // Add the index to the ASCII value of 'A' and convert to a character.
    (b'A' + index as u8) as char
}

impl GomokuApp {
    // rfd doesn't work for wasm
    #[cfg(not(target_arch = "wasm32"))]
    fn save_game_as(&self) {
        // Specify a default filename and a filter for JSON files
        let default_path = "savegame.json";
        if let Some(path) = rfd::FileDialog::new()
            .set_directory(".")
            .set_file_name(default_path)
            .add_filter("JSON files", &["json"])
            .save_file()
        {
            let serialized = serde_json::to_string(&self).unwrap();
            std::fs::write(path, serialized).expect("Unable to save game");
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn load_game_browse(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .set_directory(".")
            .add_filter("JSON files", &["json"])
            .pick_file()
        {
            let contents = std::fs::read_to_string(path).expect("Unable to load game");
            let loaded_game: GomokuApp =
                serde_json::from_str(&contents).expect("Error parsing game data");
            *self = loaded_game;
        }
    }

    fn reset_game(&mut self) {
        self.board = [[None; BOARD_SIZE]; BOARD_SIZE];
        self.current_player = Player::Black;
        self.history.clear();
        self.game_state = GameState::Ongoing;
    }

    fn show_game_rules(&mut self, ctx: &egui::Context) {
        if self.show_help {
            egui::Window::new("Game Rules")
                .open(&mut self.show_help) // This binds the window's open state to show_help
                .show(ctx, |ui| {
                    ui.label("The objective of Gomoku is to be the first player to get five stones in a row, either horizontally, vertically, or diagonally.");
                    ui.label("\nPlayers take turns placing a stone of their color on an empty intersection.");
                    ui.label("\nThe game ends when one player forms an unbroken chain of five stones, or if all intersections are filled without a winner.");

                    ui.separator();

                    ui.heading("Optional Tournament Rules");
                    ui.label("To limit first move advantage, additionally consider using one of these rules:");
                    
                    ui.label("\nPro Rule: The first player's first stone must be placed in the center. The second player's first stone can go anywhere. The first player's second stone must be three intersections away from the first.");
                    ui.label("\nLong Pro Rule: Similar to Pro, but the second stone must be four intersections away.");
                    ui.label("\nSwap: The first player places three stones (two black, one white) anywhere. The second player chooses their color.");
                    ui.label("\nSwap2: The first player places three stones, two black and one white. The second player has three options: Choose white and place a second white stone, swap colors, or place two more stones (one of each color) and let the first player choose the color.");
                });
        }
    }

    fn undo(&mut self) {
        if let Some((prev_board, prev_player, _)) = self.history.pop() {
            self.board = prev_board;
            self.current_player = prev_player;
        }
        if matches!(self.game_state, GameState::WonBy(_)) {
            self.game_state = GameState::Ongoing;
        }
    }

    fn check_win(&mut self, row: usize, col: usize, player: Player) -> bool {
        let directions = [
            (1, 0),  // Horizontal
            (0, 1),  // Vertical
            (1, 1),  // Diagonal (down-right)
            (1, -1), // Diagonal (down-left)
        ];

        for &(dx, dy) in &directions {
            let mut count = 1;

            // Check one direction
            count += self.count_stones_in_direction(row, col, dx, dy, player);

            // Check the opposite direction
            count += self.count_stones_in_direction(row, col, -dx, -dy, player);

            // Check if we have a line of 5
            if count == 5 {
                self.game_state = GameState::WonBy(player); // Update the game state to reflect the win
                return true;
            }
        }
        false
    }

    fn count_stones_in_direction(
        &self,
        start_row: usize,
        start_col: usize,
        dx: isize,
        dy: isize,
        player: Player,
    ) -> usize {
        let mut count = 0;
        let mut x = start_row as isize + dx;
        let mut y = start_col as isize + dy;

        while x >= 0 && x < BOARD_SIZE as isize && y >= 0 && y < BOARD_SIZE as isize {
            if self.board[x as usize][y as usize] == Some(player) {
                count += 1;
                x += dx;
                y += dy;
            } else {
                break;
            }
        }

        count
    }
}

impl eframe::App for GomokuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Gomoku");
            });
            ui.separator();

            // Use a horizontal layout to place the game board and the move history side by side
            ui.horizontal(|ui| {
                // Game board and controls
                ui.vertical(|ui| {
                    // Specify the size of the board
                    let padding = egui::vec2(30.0, 30.0); // Add padding
                    let (board_width, board_height) = (450.0, 450.0);
                    let board_size = egui::vec2(board_width, board_height) + padding * 5.0;

                    // Painting the game board
                    let (response, painter) = ui.allocate_painter(board_size, egui::Sense::click());

                    // Adjusted calculation for both padded_rect and rect
                    let rect = response.rect.shrink2(padding); // Inner drawable area
                    let adjust_amount = rect.width() / BOARD_SIZE as f32; // Determine the amount to adjust based on cell size

                    // Adjust padded_rect by reducing its size to correctly frame the 15x15 grid
                    let adjusted_padded_rect_size = egui::vec2(
                        response.rect.width() - adjust_amount,
                        response.rect.height() - adjust_amount,
                    );
                    let adjusted_padded_rect =
                        egui::Rect::from_min_size(response.rect.min, adjusted_padded_rect_size);

                    // Similarly, adjust rect to ensure it represents the playable area accurately
                    let adjusted_rect_size =
                        egui::vec2(rect.width() - adjust_amount, rect.height() - adjust_amount);
                    let adjusted_rect = egui::Rect::from_min_size(rect.min, adjusted_rect_size);

                    // Draw the backgrounds using the adjusted rects
                    painter.rect_filled(
                        adjusted_padded_rect,
                        0.0,
                        egui::Color32::from_rgb(193, 154, 107),
                    ); // Outer background
                    painter.rect_filled(adjusted_rect, 0.0, egui::Color32::from_rgb(156, 102, 31)); // Inner board area

                    // Grid lines
                    let cell_size = rect.width().min(rect.height()) / BOARD_SIZE as f32;
                    for i in 0..BOARD_SIZE {
                        let start = rect.left_top() + egui::vec2(i as f32 * cell_size, 0.0);
                        let end = start + egui::vec2(0.0, cell_size * (BOARD_SIZE as f32 - 1.0));
                        painter.line_segment([start, end], (1.0, egui::Color32::BLACK));

                        let start = rect.left_top() + egui::vec2(0.0, i as f32 * cell_size);
                        let end = start + egui::vec2(cell_size * (BOARD_SIZE as f32 - 1.0), 0.0);
                        painter.line_segment([start, end], (1.0, egui::Color32::BLACK));
                    }

                    // Guide points for the board
                    let points = vec![
                        (BOARD_SIZE / 2, BOARD_SIZE / 2), // Center point
                        (3, 3), (3, BOARD_SIZE - 4),      // Corner points
                        (BOARD_SIZE - 4, 3), (BOARD_SIZE - 4, BOARD_SIZE - 4),
                    ];
                    let point_radius = cell_size / 10.0; // Size of the point
                    for &(x, y) in &points {
                        let center = rect.left_top() + egui::vec2(y as f32 * cell_size, x as f32 * cell_size);
                        painter.circle_filled(center, point_radius, egui::Color32::BLACK);
                    }

                    // Row indicators (Numbers), positioned to the left of the board
                    for i in 0..BOARD_SIZE {
                        let text = format!("{}", i + 1); // Rows numbered from 1 to BOARD_SIZE
                        let x_pos = rect.left() - 5.0; // A fixed offset to the left of the board
                        let y_pos = rect.top() + i as f32 * cell_size + cell_size / 2.0 - 15.0;
                        painter.text(
                            egui::pos2(x_pos, y_pos),
                            egui::Align2::RIGHT_CENTER,
                            text,
                            egui::FontId::default(),
                            egui::Color32::BLACK,
                        );
                    }

                    for i in 0..BOARD_SIZE {
                        let text = column_index_to_letter(i).to_string(); // Convert column number to letter
                        let x_pos = rect.left() + i as f32 * cell_size + cell_size / 2.0 - 15.0; 
                        let y_pos = rect.bottom() - 30.0;
                        painter.text(
                            egui::pos2(x_pos, y_pos),
                            egui::Align2::CENTER_TOP,
                            text,
                            egui::FontId::default(),
                            egui::Color32::BLACK,
                        );
                    }

                    // Stones
                    for i in 0..BOARD_SIZE {
                        for j in 0..BOARD_SIZE {
                            if let Some(player) = self.board[i][j] {
                                // Adjust to draw stones on the intersections
                                let center = rect.left_top()
                                    + egui::vec2(j as f32 * cell_size, i as f32 * cell_size);
                                let stone_radius = cell_size / 2.5;

                                let color = match player {
                                    Player::Black => egui::Color32::BLACK,
                                    Player::White => egui::Color32::WHITE,
                                };

                                let shadow_offset = egui::vec2(2.0, 2.0);
                                let shadow_center = center + shadow_offset;
                                let shadow_color =
                                    egui::Color32::from_rgba_unmultiplied(0, 0, 0, 100); // Dark gray shadow with alpha

                                painter.circle_filled(
                                    shadow_center,
                                    stone_radius * 1.1,
                                    shadow_color,
                                ); // Draw shadow
                                painter.circle_filled(center, stone_radius, color);
                            }
                        }
                    }
                    if matches!(self.game_state, GameState::WonBy(_)) {
                        // Display winner notification
                        // Allow utility actions (save, load, reset, undo) but prevent new moves
                        egui::Window::new("Game Over")
                            .title_bar(true)
                            .show(ctx, |ui| match self.game_state {
                                GameState::WonBy(Player::Black) => ui.label("Black wins!"),
                                GameState::WonBy(Player::White) => ui.label("White wins!"),
                                GameState::Ongoing => todo!(),
                                GameState::Draw => todo!(),
                            });
                    } else {
                        // Detect clicks on the board to place stones
                        if response.clicked() {
                            let click_pos =
                                response.interact_pointer_pos().unwrap() - rect.left_top();

                            // Calculate the row and column indices
                            let row =
                                ((click_pos.y + cell_size / 2.0) / cell_size).floor() as usize;
                            let col =
                                ((click_pos.x + cell_size / 2.0) / cell_size).floor() as usize;

                            if row < BOARD_SIZE
                                && col < BOARD_SIZE
                                && self.board[row][col].is_none()
                            {
                                // Save the current state before making the move
                                self.history
                                    .push((self.board, self.current_player, (row, col)));

                                self.board[row][col] = Some(self.current_player);

                                if self.check_win(row, col, self.current_player) {
                                    println!("{:?} wins!", self.current_player);
                                }

                                self.current_player = match self.current_player {
                                    Player::Black => Player::White,
                                    Player::White => Player::Black,
                                };
                            }
                        }
                    }
                    // Undo and Reset buttons
                    ui.horizontal(|ui| {
                        if ui.button("Undo").clicked() {
                            self.undo();
                        }
                        if ui.button("Reset").clicked() {
                            self.reset_game();
                        }
                        if ui.button("Help").clicked() {
                            self.show_help = true;
                        }

                        #[cfg(not(target_arch = "wasm32"))]{
                            if ui.button("Save Game").clicked() {
                                self.save_game_as();
                            }
                            if ui.button("Load Game").clicked() {
                                self.load_game_browse();
                            }
                        }           
                     });
                });

                self.show_game_rules(ctx);

                // Move history display to the right of the game board
                ui.vertical(|ui| {
                    ui.heading("Move History");
                    let move_pairs = self.history.chunks(2);
                    let mut turn_number = 1;
                
                    for pair in move_pairs {
                        match pair {
                            [black_move, white_move] => {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{}.", turn_number));
                                    let black_col_letter = column_index_to_letter(black_move.2 .1);
                                    ui.label(format!(
                                        "Black: ({}, {})",
                                        black_col_letter,
                                        black_move.2 .0 + 1
                                    ));
                                    let white_col_letter = column_index_to_letter(white_move.2 .1);
                                    ui.label(format!(
                                        "White: ({}, {})",
                                        white_col_letter,
                                        white_move.2 .0 + 1
                                    ));
                                });
                                turn_number += 1;
                            }
                            [last_move] => {
                                let col_letter = column_index_to_letter(last_move.2 .1);
                                let label = match last_move.1 {
                                    Player::Black => {
                                        format!("{}.   ", turn_number)
                                            + &format!("Black: ({}, {})", col_letter, last_move.2 .0 + 1)
                                    }
                                    Player::White => {
                                        format!("{}.   ", turn_number)
                                            + &format!("White: ({}, {})", col_letter, last_move.2 .0 + 1)
                                    }
                                };
                                ui.label(label);
                            }
                            _ => {}
                        }
                    }
                });
            });
        });
    }
}

// native app
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Gomoku Game",
        options,
        Box::new(|_cc| Box::new(GomokuApp::default())),
    );
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    let options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                options,
                Box::new(|_cc| Box::new(GomokuApp::default())),
            )
            .await
            .expect("failed to start eframe");
    });
}
