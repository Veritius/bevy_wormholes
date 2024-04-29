#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var main_sampler: sampler;
@group(2) @binding(1) var main_texture: texture_2d<f32>;
@group(2) @binding(2) var stencil_sampler: sampler;
@group(2) @binding(3) var stencil_texture: texture2d<f32>;

@fragment
fn fragment(
    mesh: VertexOutput
) -> @location(0) vec4<f32> {
    var color = textureSample(main_texture, main_sampler, mesh.uv);
    var mask = textureSample(stencil_texture, stencil_sampler, mesh.uv);
    return vec4<f32>(color * mask);
}