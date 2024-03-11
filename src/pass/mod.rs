use std::time::Duration;

use wgpu::{CommandEncoder, Device, Queue};

use crate::texture_store::TextureResolver;

pub mod blit;
pub mod jumpflood;
pub mod points_pass;

pub trait Pass {
    fn render(
        &mut self,
        aspect_ratio: f32,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        textures: &TextureResolver,
        elapsed: Duration,
    );
}
