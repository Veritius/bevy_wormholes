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
#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Debug, Component)]
pub struct Wormhole {
    /// The other side of the wormhole.
    pub counterpart: Entity,
}

/// Shader for drawing wormhole surfaces.
#[derive(Debug, Clone, TypePath, Asset, AsBindGroup)]
pub struct WormholeShader {
    /// The texture that the camera renders to.
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,

    /// A stencil mask that hides parts of the texture.
    #[texture(2)]
    #[sampler(3)]
    pub stencil: Option<Handle<Image>>,
}

impl WormholeShader {
    /// The asset path for the shader.
    pub const SHADER_ASSET_PATH: &'static str = "embedded://bevy_wormholes/surface.wgsl";
}

impl Material for WormholeShader {
    fn vertex_shader() -> ShaderRef {
        Self::SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}