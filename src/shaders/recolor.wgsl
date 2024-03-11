struct VertexOutput {
    @location(0) tex_coord: vec2<f32>,
    @builtin(position) position: vec4<f32>,
};


@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
) -> VertexOutput {
    var result: VertexOutput;
    result.tex_coord = tex_coords;
    result.position = vec4<f32>(position, 0.0, 1.0);
    return result;
}

@group(0)
@binding(0)
var r_color: texture_2d<f32>;

@group(0)
@binding(1)
var r_pos: texture_2d<f32>;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let p = vec2<u32>(textureLoad(r_pos, vec2<u32>(vertex.position.xy), 0).xy);
    let tex = textureLoad(r_color, p, 0);
    return tex;
}