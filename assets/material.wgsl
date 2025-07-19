#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct FragOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32
}

@fragment
fn fragment(mesh: VertexOutput) -> FragOut {
    var out: FragOut;
    var color = mesh.color;
    
    var t = color.a;
    out.depth = t; // color.a;
    color.a = 1.0;

    out.color = color;

    return out;
}


