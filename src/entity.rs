use std::fmt::Debug;

use specs::{Builder, World, WorldExt};

use crate::component::position::Position;
use crate::component::renderable::Renderable;
use crate::component::{CrateObj, CrateSpot, Immovable, Movable, Player, Wall};
use crate::component::box_colour::BoxColour;

pub fn wall(world: &mut World, position: &Position) {
    world
        .create_entity()
        .with(position.clone())
        .with(Renderable::new_static("/images/wall.png".to_string(), 1))
        .with(Wall {})
        .with(Immovable {})
        .build();
}

pub fn floor(world: &mut World, position: &Position) {
    world
        .create_entity()
        .with(position.clone())
        .with(Renderable::new_static("/images/floor.png".to_string(), 0))
        .build();
}

pub fn crate_obj(world: &mut World, position: &Position, colour: BoxColour) {
    let img_path = format!("/images/{}_crate.png", colour);

    world
        .create_entity()
        .with(position.clone())
        .with(Renderable::new_static(img_path, 3))
        .with(CrateObj { colour })
        .with(Movable {})
        .build();
}

pub fn crate_spot(world: &mut World, position: &Position, colour: BoxColour) {
    let img_path = format!("/images/{}_crate_spot.png", colour);

    world
        .create_entity()
        .with(position.clone())
        .with(Renderable::new_static(img_path, 2))
        .with(CrateSpot { colour })
        .build();
}

pub fn player(world: &mut World, position: &Position) {
    world
        .create_entity()
        .with(position.clone())
        .with(Renderable::new_animated(
            vec![
                "/images/player_1.png".to_string(),
                "/images/player_2.png".to_string(),
                "/images/player_3.png".to_string(),
                "/images/player_4.png".to_string(),
            ],
            4,
        ))
        .with(Player {})
        .with(Movable {})
        .build();
}
