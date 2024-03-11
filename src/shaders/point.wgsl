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

struct FragmentOutput{
    @location(0) posbuf: vec4<f32>,
    @location(1) colorbuf: vec4<f32>,

}

@fragment
fn fs_main(vertex: VertexOutput) -> FragmentOutput {
    var result: FragmentOutput;
    result.posbuf = vec4<f32>(vertex.position.xyz, 1.0);
    result.colorbuf = vec4<f32>(vertex.color, 1.0);
    return result;
}