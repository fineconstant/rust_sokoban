extern crate log;

use ggez::event::EventHandler;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::{timer, Context, GameResult};
use log::*;
use specs::{RunNow, World, WorldExt};

use crate::resource::{DeltaAccumulator, KeyDownQueue};
use crate::system::input_system::InputSystem;
use crate::system::rendering_system::RenderingSystem;
use crate::system::sokoban_game_state_system::SokobanGameStateSystem;
use crate::{component, configuration, resource};
use crate::system::event_system::EventSystem;

pub struct SokobanGame {
    world: World,
}

impl SokobanGame {
    pub fn create(context: &mut Context) -> SokobanGame {
        info!("Creating game world");
        let mut world = World::new();
        component::register_all(&mut world);
        resource::register_all(&mut world);
        configuration::initialize_level(&mut world);
        configuration::initialize_sounds(&world, context);

        SokobanGame { world }
    }
}

impl EventHandler for SokobanGame {
    fn update(&mut self, context: &mut Context) -> GameResult {
        {
            let mut input_system = InputSystem {};
            input_system.run_now(&self.world);
        }
        {
            let mut sokoban_game_state_system = SokobanGameStateSystem {};
            sokoban_game_state_system.run_now(&self.world);
        }
        {
            let mut time = self.world.write_resource::<DeltaAccumulator>();
            time.value += timer::delta(context);
        }
        {
            let mut event_system = EventSystem {};
            event_system.run_now(&self.world);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        // todo: do not create in each cycle
        let mut rendering_system = RenderingSystem::new(context);
        rendering_system.run_now(&self.world);

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        info!("Key down [{:?}]", keycode);
        self.world
            .write_resource::<KeyDownQueue>()
            .keys
            .push_back(keycode)
    }
}
