# Flappy Yandex Rover

A simple Flappy Bird-style game featuring a Yandex Rover as the main character. This game is built with the [Bevy engine](https://bevyengine.org/) in Rust. The goal is to navigate the rover through pairs of columns without colliding.

## How to Play

*   **Jump:** Press the `Space` key or the `Left Mouse Button` to make the rover ascend.
*   **Objective:** Guide the rover through the gaps between the columns to score points.
*   **Game Over:** The game ends if the rover collides with a column or hits the ground.
*   **Restart:** After a game over, press any key to restart.

## Getting Started

### Prerequisites

Ensure you have the Rust toolchain (including `cargo`) installed on your system. You can find installation instructions at [rustup.rs](https://rustup.rs/).

### Installation & Running

1.  Clone the repository to your local machine:
    ```bash
    git clone https://github.com/crustypub/flappy-yandex-rover.git
    ```

2.  Navigate into the project directory:
    ```bash
    cd flappy-yandex-rover
    ```

3.  Run the game using Cargo:
    ```bash
    cargo run
    ```
    The first time you run the project, Cargo will download and compile all the dependencies, which may take a few minutes. Subsequent runs will be much faster.

## Project Structure

The repository is organized as follows:

*   `src/main.rs`: The main entry point for the application which launches the game.
*   `src/game.rs`: Contains all the core game logic, including entity setup, player input, movement physics, collision detection, scoring, and the game loop.
*   `assets/`: Directory containing all game assets like images for the rover (`rover.png`), columns (`column.png`), and background (`background.png`).
*   `Cargo.toml`: The Rust project manifest file, defining dependencies and project metadata.
*   `LICENSE`: The MIT license file for the project.

## Dependencies

This project relies on the following main Rust crates:

*   `bevy`: A refreshingly simple data-driven game engine.
*   `rand`: A library for random number generation, used here to create variation in column heights.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
