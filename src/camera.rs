use bevy::prelude::*;
use crate::Wormhole;

/// The main camera used in wormhole rendering.
/// Wormholes will be rendered to look correct from this entity's point of view.
#[derive(Debug, Component, Reflect)]
#[reflect(Debug, Component)]
pub struct WormholeObserver;

/// A camera that renders wormhole surfaces.
/// Exists as the counterpart to a [`WormholeObserver`].
/// 
/// Entities with [`WormholeCamera`] cannot be a child entity of another entity.
/// If this occurs, it'll be detected, an error will be logged, and the parent will be detached.
#[derive(Debug, Component, Reflect)]
#[reflect(Debug, Component)]
pub struct WormholeCamera {
    /// The entity IDs of the wormhole pair.
    pub wormholes: [Entity; 2],
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
    observers: Query<&GlobalTransform, (With<WormholeObserver>, Without<WormholeCamera>)>,
    mut cameras: Query<(&WormholeCamera, &mut Transform, &mut GlobalTransform), Without<Wormhole>>,
    wormholes: Query<&GlobalTransform, (With<Wormhole>, Without<WormholeCamera>)>,
) {
    for (camera, mut transform, mut global_transform) in cameras.iter_mut() {

    }
}