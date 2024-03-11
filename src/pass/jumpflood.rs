use std::borrow::Cow;

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupLayout, Buffer, Device, Sampler, TextureFormat,
};

use crate::{material::Material, texture_store::TextureHandle};

use super::Pass;

pub struct JumpfloodPass {
    input_texture: TextureHandle,
    output_texture: TextureHandle,
    bind_group_layout: BindGroupLayout,
    material: Material,
    bind_group: Option<wgpu::BindGroup>,
    sampler: Sampler,
    vertex_buffer: Buffer,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct JumpfloodVertex {
    position: nalgebra::Vector2<f32>,
    tex_coords: nalgebra::Vector2<f32>,
}

impl JumpfloodPass {
    pub fn new(
        device: &Device,
        input_texture: TextureHandle,
        output_texture: TextureHandle,
        output_format: TextureFormat,
        jump: u32,
    ) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        });

        let material = Self::create_material(device, output_format, &bind_group_layout, jump);

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("jumpflood sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        });

        let vertices: &[JumpfloodVertex] = &[
            JumpfloodVertex {
                position: nalgebra::Vector2::new(-1.0, 1.0),
                tex_coords: nalgebra::Vector2::new(0.0, 0.0),
            },
            JumpfloodVertex {
                position: nalgebra::Vector2::new(-1.0, -1.0),
                tex_coords: nalgebra::Vector2::new(0.0, 1.0),
            },
            JumpfloodVertex {
                position: nalgebra::Vector2::new(1.0, 1.0),
                tex_coords: nalgebra::Vector2::new(1.0, 0.0),
            },
            JumpfloodVertex {
                position: nalgebra::Vector2::new(1.0, 1.0),
                tex_coords: nalgebra::Vector2::new(1.0, 0.0),
            },
            JumpfloodVertex {
                position: nalgebra::Vector2::new(-1.0, -1.0),
                tex_coords: nalgebra::Vector2::new(0.0, 1.0),
            },
            JumpfloodVertex {
                position: nalgebra::Vector2::new(1.0, -1.0),
                tex_coords: nalgebra::Vector2::new(1.0, 1.0),
            },
        ];

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("jumpflood vertex buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            input_texture,
            output_texture,
            material,
            bind_group_layout,
            sampler,
            vertex_buffer,
            bind_group: None,
        }
    }

    fn create_material(
        device: &Device,
        format: TextureFormat,
        bind_group_layout: &BindGroupLayout,
        jump: u32
    ) -> Material {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("jumpflood pipeline layout"),
            bind_group_layouts: &[bind_group_layout],
            push_constant_ranges: &[],
        });

        let shadersource = include_str!("../shaders/jumpflood.wgsl");

        let shadersource = shadersource.replace("{JUMP}", &jump.to_string());

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("jumpflood shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Owned(shadersource)),
        });

        let vertex_buffer = [wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<JumpfloodVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 8,
                    shader_location: 1,
                },
            ],
        }];

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("jumpflood pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &vertex_buffer,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(format.into())],
            }),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            multiview: None,
        });

        Material {
            shader,
            pipeline_layout,
            render_pipeline,
        }
    }
}

impl Pass for JumpfloodPass {
    fn render(
        &mut self,
        _: f32,
        device: &wgpu::Device,
        _: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        textures: &crate::texture_store::TextureResolver,
        _: std::time::Duration,
    ) {
        let view = textures.resolve(self.input_texture);
        let output_view = textures.resolve(self.output_texture);
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("jumpflood pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("jumpflood bind group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });

        self.bind_group = Some(bind_group);

        rpass.set_bind_group(0, (&self.bind_group).as_ref().unwrap(), &[]);
        rpass.set_pipeline(&self.material.render_pipeline);
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.draw(0..6, 0..1);
    }
}
