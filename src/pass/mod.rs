use std::time::Duration;

use wgpu::{CommandEncoder, Queue};

use crate::texture_store::TextureResolver;

pub mod points_pass;

pub trait Pass {
    fn render(
        &self,
        aspect_ratio: f32,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        textures: &TextureResolver,
        elapsed: Duration
    );
}
