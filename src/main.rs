use ggez::{conf, event, GameResult};
use log::*;

use rust_sokoban::infrastructure::logging;
use rust_sokoban::sokoban_game::SokobanGame;

pub fn main() -> GameResult {
    logging::init();

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("Rust Sokoban", "Kamil Duda")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800f32, 800f32))
        .add_resource_path("./resources");

    let (context, event_loop) = &mut context_builder.build()?;
    // Create the game state
    let sokoban_game = &mut SokobanGame::create();
    // Run the main event loop
    event::run(context, event_loop, sokoban_game)
}
