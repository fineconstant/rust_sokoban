use crate::component::position::Position;
use crate::component::{CrateObj, CrateSpot};
use crate::resource::{GameState, SokobanGameState};
use specs::{Join, ReadStorage, System, Write};
use std::collections::HashMap;

pub struct SokobanGameStateSystem;

impl<'a> System<'a> for SokobanGameStateSystem {
    type SystemData = (
        Write<'a, SokobanGameState>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, CrateObj>,
        ReadStorage<'a, CrateSpot>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut game_state, positions, crates, spots) = data;

        let crates_by_position = (&positions, &crates).join().collect::<HashMap<_, _>>();

        for (position, spot) in (&positions, &spots).join() {
            if let Some(x) = crates_by_position.get(&position) {}

            match crates_by_position.get(&position) {
                Some(crate_obj) => {
                    if crate_obj.colour != spot.colour {
                        game_state.state = GameState::Playing;
                        return;
                    }
                }
                None => {
                    game_state.state = GameState::Playing;
                    return;
                }
            }

            game_state.state = GameState::Won;
        }
    }
}
