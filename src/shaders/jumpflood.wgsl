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
var r_sampler: sampler;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let posf = vertex.position.xy;
    let pos = vec2<u32>(posf);
    var t: vec4<f32> = textureLoad(r_color, pos, 0);

    let tn = textureLoad(r_color, pos+vec2<u32>(0, {JUMP}), 0);
    let ts = textureLoad(r_color, pos-vec2<u32>(0, {JUMP}), 0);
    let tw = textureLoad(r_color, pos+vec2<u32>({JUMP}, 0), 0);
    let te = textureLoad(r_color, pos-vec2<u32>({JUMP}, 0), 0);
    if(tn.a > 0.5 && length(t.xy-posf) > length(tn.xy-posf) ){
        t = tn;
    }
    if(ts.a > 0.5 && length(t.xy-posf) > length(ts.xy-posf) ){
        t = ts;
    }
    if(tw.a > 0.5 && length(t.xy-posf) > length(tw.xy-posf) ){
        t = tw;
    }
    if(te.a > 0.5 && length(t.xy-posf) > length(te.xy-posf) ){
        t = te;
    }
    
    if(length(t.xy-posf) > 4.0){
        return vec4<f32>(0);
    }

    return t;
}