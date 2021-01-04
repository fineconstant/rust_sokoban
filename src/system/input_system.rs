use std::collections::{BTreeMap, HashMap};

use ggez::input::gamepad::gilrs::ev::Button::LeftThumb;
use ggez::input::keyboard::KeyCode;
use ggez::input::mouse::position;
use ggez::nalgebra::{inf, Vector};
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};

use crate::component::position::Position;
use crate::component::{Immovable, Movable, Player};
use crate::error::SokobanResult;
use crate::event::GameEvent;
use crate::resource::{EventQueue, GameState, KeyDownQueue, MovesCount};

trait DirectionMapCheck {
    fn generate(&self) -> BTreeMap<u8, Position>;
}

struct NorthMapCheck<'a> {
    position: &'a Position,
}

struct SouthMapCheck<'a> {
    position: &'a Position,
}

struct WestMapCheck<'a> {
    position: &'a Position,
}

struct EastMapCheck<'a> {
    position: &'a Position,
}

impl DirectionMapCheck for NorthMapCheck<'_> {
    fn generate(&self) -> BTreeMap<u8, Position> {
        move_prediction(self.position, |mut position| position.move_north())
    }
}

impl<'a> DirectionMapCheck for SouthMapCheck<'a> {
    fn generate(&self) -> BTreeMap<u8, Position> {
        move_prediction(self.position, |mut position| position.move_south())
    }
}

impl<'a> DirectionMapCheck for WestMapCheck<'a> {
    fn generate(&self) -> BTreeMap<u8, Position> {
        move_prediction(self.position, |mut position| position.move_west())
    }
}

impl<'a> DirectionMapCheck for EastMapCheck<'a> {
    fn generate(&self) -> BTreeMap<u8, Position> {
        move_prediction(self.position, |mut position| position.move_east())
    }
}

fn move_prediction<F>(position: &Position, move_a: F) -> BTreeMap<u8, Position>
where
    F: Fn(Position) -> SokobanResult<Position>,
{
    let player_position = position.clone();
    let a_position = move_a(player_position.clone()).expect("Invalid move");
    let b_result = move_a(a_position.clone());

    let mut result = BTreeMap::new();

    result.insert(0, player_position);
    result.insert(1, a_position);

    if let Ok(b_position) = b_result {
        result.insert(2, b_position);
        return result;
    }

    result
}

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        Write<'a, KeyDownQueue>,
        Write<'a, MovesCount>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
        Write<'a, EventQueue>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            players,
            mut positions,
            mut input_queue,
            mut moves_count,
            movables,
            immovables,
            mut event_queue,
        ) = data;

        let key_pressed = input_queue.keys.pop_front();
        if key_pressed.is_none() {
            return;
        }

        if let Some(key) = key_pressed {
            match key {
                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                    let key = key_pressed.expect("Could not get Key Code");

                    let mut entities_to_move = Vec::new();

                    for (player_entity, player_position, _) in
                        (&entities, &positions, &players).join()
                    {
                        let all_movables = (&entities, &positions, &movables)
                            .join()
                            .map(|(entity, mov_position, _)| (mov_position, entity))
                            .collect::<HashMap<_, _>>();

                        let all_immovables = (&entities, &positions, &immovables)
                            .join()
                            .map(|(entity, immov_position, _)| (immov_position, entity))
                            .collect::<HashMap<_, _>>();

                        let direction_range: Box<dyn DirectionMapCheck> = match key {
                            KeyCode::Up => Box::new(NorthMapCheck {
                                position: player_position,
                            }),
                            KeyCode::Down => Box::new(SouthMapCheck {
                                position: player_position,
                            }),
                            KeyCode::Left => Box::new(WestMapCheck {
                                position: player_position,
                            }),
                            KeyCode::Right => Box::new(EastMapCheck {
                                position: player_position,
                            }),
                            _ => return,
                        };

                        entities_to_move.push(player_entity.id());

                        let positions_map = direction_range.generate();
                        info!("Positions map {:?}", positions_map);

                        let position_next = positions_map.get(&1).expect("Invalid position");
                        let o_position_next_plus_one = positions_map.get(&2);

                        let o_immovable_entity = all_immovables.get(position_next);

                        match o_immovable_entity {
                            // (movable, immovable) -> nothing
                            Some(_) => {
                                info!("Next: immovable");
                                event_queue.events.push(GameEvent::PlayerHitObstacle);
                                entities_to_move.clear();
                                break;
                            }
                            None => {}
                        }

                        let o_movable_entity = all_movables.get(position_next);
                        match o_movable_entity {
                            // (movable, movable, ?) -> ?
                            Some(movable_entity) => {
                                info!("Next: movable");
                                entities_to_move.push(movable_entity.id());

                                // (movable, movable, movable  ) -> nothing
                                if let Some(position_next_plus_one) = o_position_next_plus_one {
                                    // (movable, movable, immovable) -> nothing
                                    if let Some(_) = all_immovables.get(position_next_plus_one) {
                                        info!("Next plus one: immovable");
                                        event_queue.events.push(GameEvent::PlayerHitObstacle);
                                        entities_to_move.clear();
                                        break;
                                    }

                                    // (movable, movable, movable) -> nothing
                                    if let Some(_) = all_movables.get(position_next_plus_one) {
                                        info!("Next plus one: movable");
                                        event_queue.events.push(GameEvent::PlayerHitObstacle);
                                        entities_to_move.clear();
                                        break;
                                    }

                                    // else
                                    // (movable, movable, nothing  ) -> move
                                    info!("Next+1: nothing");
                                }
                            }
                            // (movable, nothing) -> move
                            None => info!("Next: nothing"),
                        }

                        break;

                        info!("Entities to move: {:?}", entities_to_move);
                    }

                    for &entity_id in &entities_to_move {
                        let o_position = positions.get_mut(entities.entity(entity_id));
                        if let Some(position) = o_position {
                            match key {
                                KeyCode::Up => {
                                    position.move_north();
                                }
                                KeyCode::Down => {
                                    position.move_south();
                                }
                                KeyCode::Left => {
                                    position.move_west();
                                }
                                KeyCode::Right => {
                                    position.move_east();
                                }
                                _ => {}
                            };
                        }

                        event_queue.events.push(GameEvent::EntityMoved { entity_id })
                    }

                    if !entities_to_move.is_empty() {
                        moves_count.value += 1;
                        info!("Moves count {}", moves_count.value);
                    }
                }
                _ => {}
            }
        }
    }
}
