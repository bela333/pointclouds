use bytemuck::{Pod, Zeroable};
use nalgebra::Vector3;
use wgpu::{util::DeviceExt, Buffer, Device, RenderPass, TextureFormat};

use crate::material::{self, Material};

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
    pub fn new(device: &Device, format: TextureFormat, vertices: Vec<BasicVertex>) -> Self {
        let buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<BasicVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                // Color
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 12,
                    shader_location: 1,
                },
            ],
        };

        let material = material::Material::create(
            device,
            format,
            "basic",
            &[buffer_layout],
            include_str!("shader.wgsl"),
        );

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

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
        pass.set_pipeline(self.material.get_render_pipeline());
        pass.set_vertex_buffer(0, self.buffer.slice(..));
        pass.draw(0..self.vertex_count, 0..1);
    }
}
