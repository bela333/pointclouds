use wgpu::{PipelineLayout, ShaderModule};

pub struct Material {
    #[allow(dead_code)]
    pub shader: ShaderModule,
    #[allow(dead_code)]
    pub pipeline_layout: PipelineLayout,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl Material {}
