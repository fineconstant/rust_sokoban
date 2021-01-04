use log::*;
use specs::World;

use crate::component::*;
use crate::component::box_colour::BoxColour;
use crate::component::position::Position;
use crate::entity;
use crate::entity::{crate_obj, crate_spot, floor, player, wall};

pub const TILE_EDGE_SIZE: usize = 64;

const LEVEL_MAP: &str = "
    W W W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . . . . . W
    W . P . . . . W
    W . RB . . . . W
    W . . BS . RS . W
    W . . . . . . W
    W W W W W W W W
    ";

pub fn initialize_level(world: &mut World) {
    info!("Initializing level");
    load_map(world, LEVEL_MAP);
}

fn load_map(world: &mut World, map: &str) {
    info!("Loading map");
    for (y, row) in map.trim().lines().map(|x| x.trim()).enumerate() {
        info!("{} {}", y, row);
        for (x, tile) in row.split(' ').enumerate() {
            let position = &Position::new(x, y);
            match tile {
                "." => entity::floor(world, position),
                "P" => {
                    entity::floor(world, position);
                    entity::player(world, position);
                }
                "W" => {
                    entity::floor(world, position);
                    entity::wall(world, position);
                }
                "BB" => {
                    entity::floor(world, position);
                    entity::crate_obj(world, position, BoxColour::Blue);
                }
                "RB" => {
                    entity::floor(world, position);
                    entity::crate_obj(world, position, BoxColour::Red);
                }
                "BS" => {
                    entity::floor(world, position);
                    entity::crate_spot(world, position, BoxColour::Blue);
                }
                "RS" => {
                    entity::floor(world, position);
                    entity::crate_spot(world, position, BoxColour::Red);
                }
                item => {
                    panic!(
                        "Unrecognized map item [{}] in position [{}, {}]",
                        item, y, x
                    )
                }
            }
        }
    }
}
