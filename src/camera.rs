use bevy::{prelude::*, render::camera::RenderTarget};
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
    _hidden: (), // prevent manual construction
}

impl WormholeCameraBundle {
    /// Creates a new [`WormholeCameraBundle`] that renders to `handle` at order `-1`.
    pub fn new(
        target: Entity,
        handle: Handle<Image>,
    ) -> Self {
        Self {
            camera: Camera3dBundle {
                camera: Camera {
                    order: -1,
                    target: RenderTarget::Image(handle),
                    ..default()
                },
                ..default()
            },
            comp: WormholeCamera { target },
            _hidden: (),
        }
    }

    /// Creates a new [`WormholeCameraBundle`] that renders to `handle` at order `-1`.
    pub fn new_with_order(
        target: Entity,
        handle: Handle<Image>,
        order: isize,
    ) -> Self {
        let mut bundle = Self::new(target, handle);
        bundle.camera.camera.order = order;
        return bundle;
    }
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
    mut cameras: Query<(&WormholeCamera, &mut Transform, &mut GlobalTransform), Without<Wormhole>>,
    wormholes: Query<&GlobalTransform, (With<Wormhole>, Without<WormholeCamera>)>,
) {
    for (camera, mut transform, mut global_transform) in cameras.iter_mut() {
        todo!()
    }
}