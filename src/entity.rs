use specs::{Builder, World, WorldExt};

use crate::component::position::Position;
use crate::component::{
    BoxColour, CrateObj, CrateSpot, Immovable, Movable, Player, Renderable, Wall,
};
use std::fmt::Debug;

pub fn wall(world: &mut World, position: &Position) {
    world
        .create_entity()
        .with(position.clone())
        .with(Renderable {
            img_path: "/images/wall.png".to_string(),
            z_index: 1,
        })
        .with(Wall {})
        .with(Immovable {})
        .build();
}

pub fn floor(world: &mut World, position: &Position) {
    world
        .create_entity()
        .with(position.clone())
        .with(Renderable {
            img_path: "/images/floor.png".to_string(),
            z_index: 0,
        })
        .build();
}

pub fn crate_obj(world: &mut World, position: &Position, colour: BoxColour) {
    let img_path = format!("/images/{}_crate.png", colour);

    world
        .create_entity()
        .with(position.clone())
        .with(Renderable {
            img_path,
            z_index: 3,
        })
        .with(CrateObj { colour })
        .with(Movable {})
        .build();
}

pub fn crate_spot(world: &mut World, position: &Position, colour: BoxColour) {
    let img_path = format!("/images/{}_crate_spot.png", colour);

    world
        .create_entity()
        .with(position.clone())
        .with(Renderable {
            img_path,
            z_index: 2,
        })
        .with(CrateSpot { colour })
        .build();
}

pub fn player(world: &mut World, position: &Position) {
    world
        .create_entity()
        .with(position.clone())
        .with(Renderable {
            img_path: "/images/player.png".to_string(),
            z_index: 4,
        })
        .with(Player {})
        .with(Movable {})
        .build();
}
