use wgpu::CommandEncoder;

use crate::texture_store::TextureResolver;

pub mod points_pass;

pub trait Pass {
    fn render(&self, encoder: &mut CommandEncoder, textures: &TextureResolver);
}
