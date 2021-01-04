use ggez::graphics;
use ggez::graphics::{Color, DrawParam, FilterMode, Image, Text};
use ggez::nalgebra::Point2;
use ggez::Context;
use specs::{Join, Read, ReadStorage, System};

use crate::component::position::Position;
use crate::component::renderable::{Renderable, RenderableKind};
use crate::configuration::TILE_EDGE_SIZE;
use crate::resource::{DeltaAccumulator, MovesCount, SokobanGameState};
use std::time::Duration;
use ggez::mint::IntraXYZ;

pub struct RenderingSystem<'a> {
    context: &'a mut Context,
}

impl RenderingSystem<'_> {
    pub fn new(context: &mut Context) -> RenderingSystem {
        RenderingSystem { context }
    }
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Read<'a, SokobanGameState>,
        Read<'a, MovesCount>,
        Read<'a, DeltaAccumulator>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (game_state, moves_count, delta_acc, renderables, positions) = data;

        graphics::clear(self.context, Color::from_rgba(89, 106, 108, 255));

        let mut rendering_data = (&renderables, &positions).join().collect::<Vec<_>>();
        rendering_data.sort_by(|&x, &y| x.0.z_index.partial_cmp(&y.0.z_index).unwrap());

        rendering_data.iter().for_each(|&(renderable, position)| {
            let image = self.get_frame(renderable, delta_acc.value);

            // let image =
            //     Image::new(self.context, renderable.frame(0).clone()).expect("Could open an image");
            let x = (position.point.x * TILE_EDGE_SIZE) as f32;
            let y = (position.point.y * TILE_EDGE_SIZE) as f32;
            let destination = Point2::from([x, y]);

            let params = DrawParam::new().dest(destination);
            graphics::draw(self.context, &image, params).expect("Could not draw an image");
        });

        self.draw_text(&game_state.state.to_string(), 544, 16);
        self.draw_text(&moves_count.to_string(), 544, 32);

        graphics::present(self.context).expect("Could not present graphics")
    }
}

impl RenderingSystem<'_> {
    fn draw_text(&mut self, text: &str, x: usize, y: usize) {
        let text = Text::new(text);
        let dimensions = Point2::from([0.0, 0.0]);
        let colour = Some(Color::from_rgba(255, 255, 255, 255));
        let destination = Point2::from([x as f32, y as f32]);

        graphics::queue_text(self.context, &text, dimensions, colour);
        graphics::draw_queued_text(
            self.context,
            DrawParam::new().dest(destination),
            None,
            FilterMode::Linear,
        )
        .expect("Cloud not draw text");
    }

    fn get_frame(&mut self, renderable: &Renderable, delta_acc: Duration) -> Image {
        let idx = match renderable.kind {
            RenderableKind::Static => 0,
            RenderableKind::Animated => {
                let millis_in_second = (delta_acc.as_millis() % 1000);
                (millis_in_second / 250) as usize
            }
        };

        Image::new(self.context, renderable.frame(idx)).expect("Could open an image")
    }
}
