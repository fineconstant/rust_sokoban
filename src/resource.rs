use core::fmt;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::time::Duration;

use ggez::input::keyboard::KeyCode;
use log::*;
use specs::World;

use crate::event::GameEvent;
use ggez::audio;
use ggez::audio::SoundSource;
use specs::hibitset::{DrainBitIter, DrainableBitSet};
use std::collections::vec_deque::Drain;
use std::iter::FromIterator;

#[derive(Default)]
pub struct KeyDownQueue {
    pub keys: VecDeque<KeyCode>,
}

#[derive(Default)]
pub struct MovesCount {
    pub value: u32,
}

impl Display for MovesCount {
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
            GameState::Won => "State: Won",
        })
    }
}

#[derive(Default)]
pub struct DeltaAccumulator {
    pub value: Duration,
}

#[derive(Default)]
pub struct EventQueue {
   pub events: Vec<GameEvent>,
}

#[derive(Default)]
pub struct SoundsStore {
    pub sounds: HashMap<GameSounds, audio::Source>,
}

impl SoundsStore {
    pub fn play_sound(&mut self, sound: GameSounds) {
        let _ = self
            .sounds
            .get_mut(&sound)
            .expect("Could not find a sound")
            .play_detached();
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameSounds {
    WallHit,
    CorrectSpot,
    WrongSpot,
}

impl GameSounds {
    pub fn value(&self) -> &str {
        match self {
            GameSounds::WallHit => "wall_hit",
            GameSounds::CorrectSpot => "correct_spot",
            GameSounds::WrongSpot => "wrong_spot",
        }
    }
}

pub fn register_all(world: &mut World) {
    info!("Registering resources");
    world.insert(KeyDownQueue::default());
    world.insert(MovesCount::default());
    world.insert(SokobanGameState::default());
    world.insert(DeltaAccumulator::default());
    world.insert(EventQueue::default());
    world.insert(SoundsStore::default());
}
