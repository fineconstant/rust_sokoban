use ggez::graphics::{spritebatch, Color, DrawParam, FilterMode, Image, Text};
use ggez::nalgebra::Point2;
use ggez::Context;
use ggez::{graphics, timer};
use specs::{Join, Read, ReadStorage, System};

use crate::component::position::Position;
use crate::component::renderable::{Renderable, RenderableKind};
use crate::configuration::TILE_EDGE_SIZE;
use crate::resource::{DeltaAccumulator, MovesCount, SokobanGameState};
use ggez::mint::IntraXYZ;
use itertools::Itertools;
use std::collections::HashMap;
use std::time::Duration;

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
        let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();

        graphics::clear(self.context, Color::from_rgba(89, 106, 108, 255));

        let mut rendering_data = (&renderables, &positions).join().collect::<Vec<_>>();
        rendering_data.sort_by(|&x, &y| x.0.z_index.partial_cmp(&y.0.z_index).unwrap());

        rendering_data.iter().for_each(|&(renderable, position)| {
            let image_path = self.get_frame_path(renderable, delta_acc.value);
            let x = (position.point.x * TILE_EDGE_SIZE) as f32;
            let y = (position.point.y * TILE_EDGE_SIZE) as f32;
            let draw_param = DrawParam::new().dest(Point2::from([x, y]));

            rendering_batches
                .entry(renderable.z_index)
                .or_default()
                .entry(image_path)
                .or_default()
                .push(draw_param);
        });

        for (_z, z_group) in rendering_batches
            .iter()
            .sorted_by(|(z1, _), (z2, _)| z1.cmp(z2))
        {
            for (image_path, draw_params) in z_group {
                let image = Image::new(self.context, image_path).expect("Could not load an image");
                let mut batch = spritebatch::SpriteBatch::new(image);

                for param in draw_params {
                    batch.add(*param);
                }

                graphics::draw(self.context, &batch, DrawParam::new());
            }
        }

        self.draw_text(&format!("FPS: {:.0}", timer::fps(self.context)), 544, 16);
        self.draw_text(&game_state.state.to_string(), 544, 64);
        self.draw_text(&moves_count.to_string(), 544, 80);

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
            RenderableKind::Animated => ((delta_acc.as_millis() % 1000) / 250) as usize,
        };

        Image::new(self.context, renderable.frame(idx)).expect("Could open an image")
    }

    fn get_frame_path(&mut self, renderable: &Renderable, delta_acc: Duration) -> String {
        let idx = match renderable.kind {
            RenderableKind::Static => 0,
            RenderableKind::Animated => ((delta_acc.as_millis() % 1000) / 250) as usize,
        };

        renderable.frame(idx)
    }
}
