# Rust Sokoban

Sokoban game implementation loosely based on [Rust Sokoban book](https://sokoban.iolivia.me/).

The main objectives were to learn [GGEZ](https://ggez.rs/) game engine, [Specs ECS](https://specs.amethyst.rs/) and polish Rust lang skills.

image::github/screenshot.png[Rust Sokoban screenshot]

## How to run

- Development `cargo run`
- Production `cargo run --release`


## To do
- Do not initialize systems in each call of EventHandler::draw function. [Raise a question?](https://github.com/iolivia/rust-sokoban/issues)
- Optimize rendering - image caching
- Do not move player when key is continuously pressed
