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
    out.depth = 0.0;
    
    return out;
}
