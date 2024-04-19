use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

/// [`TextureUsages`] needed for a functional [`Image`] used in drawing wormholes.
// Using an integer is a hack to get around bitwise OR being unusable in a const context.
pub const WORMHOLE_TEXTURE_USAGES: TextureUsages = TextureUsages::from_bits_retain(22);

// Test to make sure that WORMHOLE_TEXTURE_USAGES is actually correct.
#[test]
fn correct_usages_test() {
    let k = TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT;
    assert_eq!(WORMHOLE_TEXTURE_USAGES, k);
}

/// A wormhole.
#[derive(Debug, Clone, Component, Reflect, ExtractComponent)]
#[reflect(Debug, Component)]
pub struct Wormhole {
    /// The other side of the wormhole.
    pub counterpart: Entity,
}

/// A bundle for creating a [`Wormhole`].
#[derive(Bundle)]
#[allow(missing_docs)]
pub struct WormholeBundle {
    pub transform: TransformBundle,
    pub visibility: VisibilityBundle,
    pub wormhole: Wormhole,
    pub mesh: Handle<Mesh>,
    pub material: Handle<WormholeMaterial>,
}

impl WormholeBundle {
    /// Creates a new [`Wormhole`] with an appropriate [`Mesh`] and [`Image`].
    /// 
    /// `dimensions` is in world units. `resolution` is in pixels.
    /// 
    /// [`counterpart`](Wormhole::counterpart) will be set to a placeholder value.
    /// It must be replaced with the entity ID of the other side of the wormhole.
    pub fn new(
        dimensions: Vec2,
        resolution: UVec2,
        meshes: &mut Assets<Mesh>,
        images: &mut Assets<Image>,
        materials: &mut Assets<WormholeMaterial>,
    ) -> Self {
        let image = images.add(Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: resolution.x,
                    height: resolution.y,
                    ..default()
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8Unorm,
                usage: WORMHOLE_TEXTURE_USAGES,
                view_formats: &[],
            },
            ..default()
        });

        let material = materials.add(WormholeMaterial {
            texture: image,
        });

        Self {
            transform: TransformBundle::default(),
            visibility: VisibilityBundle::default(),
            wormhole: Wormhole {
                counterpart: Entity::PLACEHOLDER,
            },
            mesh: meshes.add(Plane3d::new(Vec3::Y).mesh().size(dimensions.x, dimensions.y)),
            material,
        }
    }
}

/// A simple shader for rendering wormhole surfaces.
/// You can also use [`StandardMaterial`] with the same `Handle<Image>` if you want.
#[derive(Debug, Clone, TypePath, Asset, AsBindGroup)]
#[allow(missing_docs)]
pub struct WormholeMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
}

impl Material for WormholeMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_wormholes/surface.wgsl".into()
    }
}