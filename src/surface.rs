use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, TextureUsages};

/// [`TextureUsages`] needed for a functional [`Image`] used in drawing wormholes.
// Using an integer is a hack to get around bitwise OR being unusable in a const context.
pub const WORMHOLE_TEXTURE_USAGES: TextureUsages = TextureUsages::from_bits_retain(22);

// Test to make sure that WORMHOLE_TEXTURE_USAGES is actually correct.
#[test]
fn correct_usages_test() {
    let k = TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT;
    assert_eq!(WORMHOLE_TEXTURE_USAGES, k);
}

/// A wormhole entity. It's recommended that you use [`WormholeBuilder`](crate::WormholeBuilder) to create this component.
#[derive(Debug, Component, Reflect)]
#[reflect(Debug, Component)]
pub struct Wormhole {
    /// The other side of the wormhole.
    pub counterpart: Entity,
}

/// A shader for rendering wormhole surfaces.
/// 
/// To work correctly, the mesh's UVs must be in a linear space ranging from `[0.0, 0.0]` to `[1.0, 1.0]`.
#[derive(Debug, Clone, TypePath, Asset, AsBindGroup)]
#[allow(missing_docs)]
pub struct WormholeShader {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,

    #[texture(2)]
    #[sampler(3)]
    pub stencil: Option<Handle<Image>>,
}

impl Material for WormholeShader {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_wormholes/surface.wgsl".into()
    }
}