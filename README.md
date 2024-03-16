# Gomoku Game in Rust

This project is a Gomoku game (also known as Five in a Row) implemented in Rust using the `eframe` and `egui` libraries for the GUI, and `serde` and `serde_json` for game state serialization. The game allows two players to take turns placing stones on a 15x15 board with the goal of being the first to align five stones vertically, horizontally, or diagonally.

![Screenshot of game](img/game.png?raw=true "Title")

## Features

- An interactive game board GUI.
- Support for undoing moves and resetting the game.
- Game state persistence with save and load functionality.
- A help window that explains the game rules.

## Installation

To build this game from source, you need to have Rust and Cargo installed on your computer. If you don't have Rust installed, you can follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### Cloning the Repository

1. First, clone the repository to your local machine:

    ```sh
    git clone https://github.com/rbnyng/rust_gomoku
    ```
2. Navigate to the project directory:
    ```sh
    cd rust_gomoku
    ```

### Running the Game

To compile and run the game, use the following command from the project directory:
    ```sh
    cargo run
    ```

Or you can build it into an executable with:
    ```sh
    cargo build --release
    ```

Alternatively, run the precompiled Windows executable.

## Usage

Upon starting the game, you will be presented with a 15x15 board. The game starts with the black player's turn. Players take turns placing a stone of their color on the board by clicking an empty intersection.

### Controls

- **Undo**: Reverts the last move.
- **Reset**: Resets the game to its initial state.
- **Help**: Displays the game rules.
- **Save Game**: Saves the current game state to a file.
- **Load Game**: Loads a game state from a file.

## Optional Tournament Rules

For players looking to mitigate the first player advantage, the following tournament rules can be optionally followed:

### Pro Rule

- **First Move**: The first player's first stone must be placed in the center of the board.
- **Second Move**: The second player's first stone may be placed anywhere on the board.
- **Third Move**: The first player's second stone must be placed at least three intersections away from the first stone, meaning there should be two empty intersections between the two stones.

### Long Pro Rule

- **First Move**: Similar to the Pro rule, the first player's first stone is placed in the center of the board.
- **Second Move**: The second player's first stone may be placed anywhere on the board.
- **Third Move**: The first player's second stone must be placed at least four intersections away from the first stone, requiring three empty intersections between the two stones.

### Swap

- The tentative first player places three stones on the board (two black and one white) in any configuration.
- The tentative second player then chooses which color to play as, and the game proceeds with the second player as white playing their second stone.

### Swap2

- **Initial Placement**: The tentative first player places three stones on the board, two black and one white.
- **Second Player's Options**:
    1. Choose to play as white and place a second white stone.
    2. Swap colors, choosing to play as black.
    3. Place two more stones, one black and one white, and pass the choice of which color to play back to the tentative first player.

## License

This project is open source and available under the [MIT License](LICENSE).

