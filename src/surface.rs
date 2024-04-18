use bevy::{prelude::*, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}, texture::BevyDefault}};

/// A wormhole.
#[derive(Debug, Component, Reflect)]
#[reflect(Debug, Component)]
pub struct Wormhole {

}

/// A bundle for creating a [`Wormhole`].
#[derive(Bundle)]
#[allow(missing_docs)]
pub struct WormholeBundle {
    pub transform: TransformBundle,
    pub visibility: VisibilityBundle,
    pub wormhole: Wormhole,
    pub mesh: Handle<Mesh>,
    pub image: Handle<Image>,
}

impl WormholeBundle {
    /// Creates a new [`Wormhole`] with an appropriate [`Mesh`] and [`Image`].
    pub fn new(
        dimensions: Vec2,
        resolution: UVec2,
        meshes: &mut Assets<Mesh>,
        images: &mut Assets<Image>,
    ) -> Self {
        Self {
            transform: TransformBundle::default(),
            visibility: VisibilityBundle::default(),
            wormhole: Wormhole {},
            mesh: meshes.add(Plane3d::new(Vec3::Y).mesh().size(dimensions.x, dimensions.y)),
            image: images.add(Image::new_fill(
                Extent3d { width: resolution.x, height: resolution.y, depth_or_array_layers: 1 },
                TextureDimension::D2,
                &[0; 8],
                TextureFormat::bevy_default(),
                RenderAssetUsages::all(), // TODO: Revise this.
            )),
        }
    }
}