use std::borrow::Cow;

use wgpu::{
    BindGroupLayout, Device, PipelineLayout, RenderPipeline, ShaderModule, TextureFormat,
    VertexBufferLayout,
};

pub struct Material {
    #[allow(dead_code)]
    shader: ShaderModule,
    #[allow(dead_code)]
    pipeline_layout: PipelineLayout,
    render_pipeline: wgpu::RenderPipeline,
}

impl Material {
    pub fn create(
        device: &Device,
        format: TextureFormat,
        material_name: &str,
        vertex_buffers: &[VertexBufferLayout],
        source: &str,
        bind_group_layout: &BindGroupLayout,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(format!("{} shader", material_name).as_str()),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(source)),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                polygon_mode: wgpu::PolygonMode::Point,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            pipeline_layout,
            shader,
            render_pipeline,
        }
    }

    pub fn get_render_pipeline(&self) -> &RenderPipeline {
        &self.render_pipeline
    }
}
