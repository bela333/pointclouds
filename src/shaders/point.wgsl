struct VertexOutput {
    @location(0) color: vec3<f32>,
    @location(1) screenpos: vec2<f32>,
    @builtin(position) position: vec4<f32>,
    
};

@group(0)
@binding(0)
var<uniform> transform: mat4x4<f32>;

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
) -> VertexOutput {
    var result: VertexOutput;
    result.position = transform*vec4<f32>(position, 1.0);
    result.color = color;
    result.screenpos = position.xy;
    return result;
}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(vertex.position.xy, 1.0, 1.0);
}