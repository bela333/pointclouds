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
var r_pos: texture_2d<f32>;

@group(0)
@binding(1)
var r_sampler: sampler;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let posf = vertex.position.xy;
    let pos = vec2<u32>(posf);
    var t: vec4<f32> = textureLoad(r_pos, pos, 0);
    if(t.a == 0.0){
        t = vec4<f32>(0, 0, 1, 0);
    }

    let tn = textureLoad(r_pos, pos+vec2<u32>(0, {JUMP}), 0);
    let ts = textureLoad(r_pos, pos-vec2<u32>(0, {JUMP}), 0);
    let tw = textureLoad(r_pos, pos+vec2<u32>({JUMP}, 0), 0);
    let te = textureLoad(r_pos, pos-vec2<u32>({JUMP}, 0), 0);
    if(tn.a > 0.5 && length(t.xy-posf) > length(tn.xy-posf) && t.z >= tn.z ){
        t = tn;
    }
    if(ts.a > 0.5 && length(t.xy-posf) > length(ts.xy-posf) && t.z >= ts.z ){
        t = ts;
    }
    if(tw.a > 0.5 && length(t.xy-posf) > length(tw.xy-posf) && t.z >= tw.z ){
        t = tw;
    }
    if(te.a > 0.5 && length(t.xy-posf) > length(te.xy-posf) && t.z >= te.z ){
        t = te;
    }
    
    if(length(t.xy-posf) > 16.0/sqrt(2.0)){
        return vec4<f32>(0);
    }

    return t;
}