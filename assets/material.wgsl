#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> shader_flags: vec4<i32>;

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

    // if an active stroke, set full alpha
    // inactive strokes have alpha set to zdepth
    if(shader_flags[0] == 1) {
        color.a = 1.0;

        // this is line art
        if(shader_flags[1] == 2) {
            // actively drawn line art should be on top of everything
            out.depth = 1.0;
        }

        // this is paint tool
        if(shader_flags[1] == 1) {
            // actively drawn paint should be drawn on top of all other paints, 
            // which has depth ranges from 0 -> 0.5 (0 is far, 0.5 is near)
            out.depth = 0.5;
        }
    }

    out.color = color;

    return out;
}


