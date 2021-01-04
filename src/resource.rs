use ggez::input::keyboard::KeyCode;
use log::*;
use specs::World;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use core::fmt;

#[derive(Default)]
pub struct KeyDownQueue {
    pub keys: VecDeque<KeyCode>,
}

#[derive(Default)]
pub struct MovesCount {
    pub value: u32,
}
impl Display for MovesCount{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Moves: {}", self.value)
    }
}

#[derive(Default)]
pub struct SokobanGameState {
    pub state: GameState,
}

#[derive(Debug)]
pub enum GameState {
    Playing,
    Won,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Playing
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            GameState::Playing => "State: Playing",
            GameState::Won => "State: Won"
        })
    }
}

pub fn register_all(world: &mut World) {
    info!("Registering resources");
    world.insert(KeyDownQueue::default());
    world.insert(MovesCount::default());
    world.insert(SokobanGameState::default());
}
