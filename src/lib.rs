#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod builder;
mod camera;
mod surface;

pub mod render;

pub use builder::{WormholeBuilder, BuiltWormholeData};
pub use camera::*;
pub use surface::*;

use bevy::{prelude::*, transform::TransformSystem};

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
    }
}

/// System sets for wormhole systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum WormholeSystem {
    /// Updating transforms of entities.
    Transform,
}