#import bevy_pbr::forward_io::VertexOutput

@group(0) @binding(1) var texture: texture_2d<f32>;
@group(0) @binding(2) var sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput
) -> @location(0) vec4<f32> {
    return textureSample(texture, sampler, mesh.uv);
}