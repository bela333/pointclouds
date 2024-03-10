use bytemuck::{Pod, Zeroable};
use nalgebra::Vector3;
use wgpu::{util::DeviceExt, BindGroupLayout, Buffer, Device, RenderPass, TextureFormat};

use crate::{material::Material, pass::points_pass::PointsPass};

pub trait Object {
    fn update(&mut self);
    fn draw<'a>(&'a self, pass: &mut RenderPass<'a>);
}

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct BasicVertex {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
}

pub struct BasicObject {
    material: Material,
    buffer: Buffer,
    vertex_count: u32,
}

impl BasicObject {
    pub fn new(
        device: &Device,
        format: TextureFormat,
        bind_group_layout: &BindGroupLayout,
        vertices: Vec<BasicVertex>,
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let material = PointsPass::create_point_material(device, format, bind_group_layout);

        Self {
            material,
            buffer,
            vertex_count: vertices.len() as u32,
        }
    }
}

impl Object for BasicObject {
    fn update(&mut self) {
        // Update the object
    }

    fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        // Draw the object
        pass.set_pipeline(&self.material.render_pipeline);
        pass.set_vertex_buffer(0, self.buffer.slice(..));
        pass.draw(0..self.vertex_count, 0..1);
    }
}
