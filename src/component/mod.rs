use crate::component::position::Position;
use core::fmt;
use specs::{Component, NullStorage, VecStorage, World, WorldExt};
use std::fmt::{Display, Formatter};
use crate::component::renderable::Renderable;
use crate::component::box_colour::BoxColour;

pub mod position;
pub mod renderable;
pub mod box_colour;

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
