use bevy::prelude::*;
use crate::Wormhole;

/// A camera that renders wormhole surfaces.
/// 
/// Entities with [`WormholeCamera`] cannot be a child entity of another entity.
/// If this occurs, it'll be detected, an error will be logged, and the parent will be detached.
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

pub(super) fn camera_parent_check_system(
    mut commands: Commands,
    query: Query<Entity, (With<WormholeCamera>, Added<Parent>)>,
) {
    if query.is_empty() { return }
    for entity in query.iter() {
        error!("WormholeCamera {entity:?} had a Parent component attached");
        commands.entity(entity).remove_parent();
    }
}

pub(super) fn camera_transform_update_system(
    mut cameras: Query<(&WormholeCamera, &mut Transform, &mut GlobalTransform)>,
    wormholes: Query<&GlobalTransform, With<Wormhole>>,
) {
    for (camera, mut transform, mut global_transform) in cameras.iter_mut() {
        todo!()
    }
}