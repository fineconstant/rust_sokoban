use crate::component::position::Position;
use specs::{Component, NullStorage, VecStorage, World, WorldExt};
use std::fmt::{Display, Formatter};
use core::fmt;

pub mod position;

pub fn register_all(world: &mut World) {
    info!("Registering components");
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Wall>();
    world.register::<Player>();
    world.register::<CrateObj>();
    world.register::<CrateSpot>();
    world.register::<Movable>();
    world.register::<Immovable>();
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub img_path: String,
    pub z_index: u8,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Wall {}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct CrateObj {
    pub colour: BoxColour,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct CrateSpot {
    pub colour: BoxColour,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

#[derive(PartialEq)]
pub enum BoxColour {
    Red,
    Blue,
}

impl Display for BoxColour{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            BoxColour::Red => "red",
            BoxColour::Blue => "blue",
        })
    }
}