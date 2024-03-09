use std::time::Duration;

use wgpu::{BindGroupLayout, Buffer, CommandEncoder, Device, Queue};

use crate::{
    object::Object,
    texture_store::{TextureHandle, TextureResolver},
};

use super::Pass;

pub struct PointsPass {
    objects: Vec<Box<dyn Object>>,
    output_view: TextureHandle,
    uniform_buf: Buffer,
    bind_group: wgpu::BindGroup,
    depth_buffer: TextureHandle,
}

impl PointsPass {
    pub fn new(
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        objects: Vec<Box<dyn Object>>,
        output_view: TextureHandle,
        depth_buffer: TextureHandle,
    ) -> Self {
        let uniform_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buf.as_entire_binding(),
            }],
        });
        Self {
            objects,
            output_view,
            uniform_buf,
            bind_group,
            depth_buffer,
        }
    }
}

impl Pass for PointsPass {
    fn render(
        &self,
        aspect_ratio: f32,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        textures: &TextureResolver,
        elapsed: Duration,
    ) {
        let elapsed = elapsed.as_secs_f32();
        // Write current perspective matrix to the uniform buffer
        let perspective: nalgebra::Matrix4<f32> =
            nalgebra::Matrix4::new_perspective(aspect_ratio, 1.0, 0.1, 100.0);
        let camera_position = nalgebra::Point3::new(elapsed.cos() * 1.0, 0.0, elapsed.sin() * 1.0);
        let camera_lookat_matrix =  nalgebra::Matrix4::look_at_rh(
            &camera_position,
            &nalgebra::Point3::new(0.0, 0.0, 0.0),
            &nalgebra::Vector3::new(0.0, 1.0, 0.0),
        );
        let mx = perspective * camera_lookat_matrix;
        queue.write_buffer(&self.uniform_buf, 0, bytemuck::bytes_of(&mx));

        let view = textures.resolve(self.output_view);
        let depth_buffer = textures.resolve(self.depth_buffer);

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_buffer,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        rpass.set_bind_group(0, &self.bind_group, &[]);
        for object in &self.objects {
            object.draw(&mut rpass);
        }
    }
}
