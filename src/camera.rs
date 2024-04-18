use bevy::prelude::*;

/// A camera that renders wormhole surfaces.
#[derive(Debug, Component, Reflect)]
#[reflect(Debug, Component)]
pub struct WormholeCamera {
    /// The wormhole's entity ID.
    pub target: Entity,
}

/// A bundle for creating a [`WormholeCamera`].
#[derive(Bundle)]
#[allow(missing_docs)]
pub struct WormholeCameraBundle {
    pub camera: Camera3dBundle,
    pub comp: WormholeCamera,
}

// fn on_wormhole_addition