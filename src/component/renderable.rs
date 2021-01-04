use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    frames: Vec<String>,
    pub frames_number: usize,
    pub z_index: u8,
    pub kind: RenderableKind,
}

impl Renderable {
    pub fn new_static(frame_path: String, z_index: u8) -> Self {
        Self {
            frames: vec![frame_path],
            frames_number: 1,
            z_index,
            kind: RenderableKind::Static,
        }
    }

    pub fn new_animated(frame_paths: Vec<String>, z_index: u8) -> Self {
        Self {
            frames_number: frame_paths.len(),
            frames: frame_paths,
            z_index,
            kind: RenderableKind::Animated,
        }
    }

    pub fn frame(&self, index: usize) -> String {
        self.frames[index % self.frames_number].clone()
    }
}

pub enum RenderableKind {
    Static,
    Animated,
}
