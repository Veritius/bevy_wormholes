use bevy::prelude::*;
use bevy::render::render_resource::TextureUsages;

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