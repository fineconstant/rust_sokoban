use crate::component::position::Position;
use crate::component::{CrateObj, CrateSpot, Player};
use crate::event::GameEvent;
use crate::resource::{GameSounds, EventQueue, SoundsStore};
use log::*;
use specs::{Entities, Join, ReadStorage, System, Write};
use std::collections::HashMap;

pub struct EventSystem;

impl<'a> System<'a> for EventSystem {
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, SoundsStore>,
        Entities<'a>,
        ReadStorage<'a, CrateObj>,
        ReadStorage<'a, CrateSpot>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut event_queue, mut sounds_store, entities, crates, spots, positions) = data;

        let mut new_events = Vec::new();

        for event in event_queue.events.drain(..) {
            info!("Processing event [{:?}]", event);

            match event {
                GameEvent::PlayerHitObstacle => sounds_store.play_sound(GameSounds::WallHit),
                GameEvent::EntityMoved { entity_id } => {
                    let entity = entities.entity(entity_id);
                    if let Some(crate_obj) = crates.get(entity) {
                        let spots_by_position =
                            (&positions, &spots).join().collect::<HashMap<_, _>>();

                        let crate_position =
                            positions.get(entity).expect("Entity's position not found");

                        if let Some(spot) = spots_by_position.get(crate_position) {
                            new_events.push(GameEvent::CrateMovedToSpot {
                                is_correct_spot: spot.colour == crate_obj.colour,
                            });
                        }
                    }
                }
                GameEvent::CrateMovedToSpot { is_correct_spot } => {
                    if is_correct_spot {
                        sounds_store.play_sound(GameSounds::CorrectSpot)
                    } else {
                        sounds_store.play_sound(GameSounds::WrongSpot)
                    }
                }
            }
        }

        event_queue.events.append(&mut new_events);
    }
}
