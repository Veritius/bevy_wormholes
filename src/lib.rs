#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod camera;
mod graph;
mod surface;

pub use camera::*;
pub use surface::*;

use bevy::{asset::embedded_asset, prelude::*, transform::TransformSystem};

/// Adds wormholes.
pub struct WormholesPlugin;

impl Plugin for WormholesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Wormhole>();
        app.register_type::<WormholeCamera>();

        app.configure_sets(PostUpdate, WormholeSystem::Transform
            .after(TransformSystem::TransformPropagate));

        app.add_systems(PostUpdate, (
            camera_parent_check_system,
            camera_transform_update_system,
        ).in_set(WormholeSystem::Transform));

        embedded_asset!(app, "surface.wgsl");
    }
}

/// System sets for wormhole systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum WormholeSystem {
    /// Updating transforms of entities.
    Transform,
}