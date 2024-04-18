#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod camera;
mod surface;

pub use camera::*;
pub use surface::*;

use bevy::prelude::*;

/// Adds wormholes.
pub struct WormholesPlugin;

impl Plugin for WormholesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Wormhole>();
        app.register_type::<WormholeCamera>();
    }
}