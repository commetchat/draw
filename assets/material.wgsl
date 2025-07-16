#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var base_color_texture: texture_2d<f32>;
@group(2) @binding(1) var base_color_sampler: sampler;

struct FragOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32
}

@fragment
fn fragment(mesh: VertexOutput) -> FragOut {
    var out: FragOut;
    var color = mesh.color;  // textureSample(base_color_texture, base_color_sampler, mesh.uv);

    var t = color.a;
    out.depth = t; // color.a;
    color.a = 1.0;

    out.color = color;

    return out;
}


