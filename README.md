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

## License

This project is open source and available under the [MIT License](LICENSE).

