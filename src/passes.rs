use wgpu::{CommandEncoder, TextureView};

use crate::{
    object::Object,
    texture_store::{Reservation, TextureResolver},
};

pub trait Pass {
    fn render(&self, encoder: &mut CommandEncoder, textures: &TextureResolver);
}

pub struct PointsPass {
    objects: Vec<Box<dyn Object>>,
}

impl PointsPass {
    pub fn new(objects: Vec<Box<dyn Object>>) -> Self {
        Self { objects }
    }
}

impl Pass for PointsPass {
    fn render(&self, encoder: &mut CommandEncoder, textures: &TextureResolver) {
        let view = textures.resolve(Reservation::get_surface()); // TODO: Replace this with input texture
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        for object in &self.objects {
            object.draw(&mut rpass);
        }
    }
}
