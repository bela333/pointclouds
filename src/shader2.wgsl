@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(vec2<f32>(x, y)*0.75, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) u32 {
    return u32(10);
}