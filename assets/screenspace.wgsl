#import bevy_pbr::{
    mesh_view_bindings::view,
    forward_io::VertexOutput,
    utils::coords_to_viewport_uv,
}

@group(2) @binding(0) var texture: texture_2d<f32>;
@group(2) @binding(1) var texture_sampler: sampler;

struct FragOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> FragOut {
    var out: FragOut;

    let viewport_uv = coords_to_viewport_uv(mesh.position.xy, view.viewport);
    let color = textureSample(texture, texture_sampler, viewport_uv);

    out.color = color;

    // use the alpha channel stored in the mesh vertex color
    // to set the zdepth of this texture
    // this is used so actively drawn strokes can be drawn
    // underneath this texture as the correct depth
    out.depth = color.a; 
    out.color.a = 1.0;
    
    return out;
}
