use bevy::{pbr::NotShadowCaster, prelude::*, render::{camera::RenderTarget, render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat}, texture::BevyDefault}};

use crate::{Wormhole, WormholeCamera, WormholeShader, WORMHOLE_TEXTURE_USAGES};

/// A builder for wormholes. Automatically configures wormholes.
pub struct WormholeBuilder {
    camera: WormholeCameraConfig,
    orange: WormholeSideConfig,
    blue: WormholeSideConfig,
}

impl WormholeBuilder {
    /// Creates a new [`WormholeBuilder`].
    pub fn new() -> Self {
        Self {
            camera: WormholeCameraConfig::default(),
            orange: WormholeSideConfig::default(),
            blue: WormholeSideConfig::default(),
        }
    }

    /// Consumes the builder, applying the changes to the [`World`].
    pub fn build(self, context: WormholeBuilderContext) -> Result<BuiltWormholeData, ()> {
        build_wormholes(self, context)
    }

    /// Individually configure the wormhole camera.
    #[inline]
    pub fn camera(&mut self, func: impl Fn(&mut WormholeCameraConfig)) {
        func(&mut self.camera)
    }

    /// Individually configure the orange wormhole.
    #[inline]
    pub fn orange(&mut self, func: impl Fn(&mut WormholeSideConfig)) {
        func(&mut self.orange)
    }

    /// Individually configure the blue wormhole.
    #[inline]
    pub fn blue(&mut self, func: impl Fn(&mut WormholeSideConfig)) {
        func(&mut self.blue)
    }

    /// Configure both wormholes at the same time.
    #[inline]
    pub fn both(&mut self, func: &impl Fn(&mut WormholeSideConfig)) {
        self.orange(func);
        self.blue(func);
    }
}

pub struct WormholeSideConfig {
    transform: Transform,
    dimensions: Vec2,
    resolution: UVec2,
    mesh_asset: Option<Handle<Mesh>>,
    rend_image: Option<Handle<Image>>,
    mask_image: Option<Handle<Image>>,
}

impl WormholeSideConfig {
    // Default, but private.
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            dimensions: Vec2::splat(1.0),
            resolution: UVec2::splat(128),
            mesh_asset: None,
            rend_image: None,
            mask_image: None,
        }
    }

    /// Sets the [`Transform`] of the wormhole.
    /// Defaults to [`Transform::default()`].
    #[inline]
    pub fn with_transform(&mut self, transform: Transform) -> &mut Self {
        self.transform = transform;
        return self;
    }

    /// Sets the dimensions of the wormhole surface, in world units.
    /// Irrelevant if `with_mesh` is used. Defaults to `1.0 x 1.0`.
    #[inline]
    pub fn with_dimensions(&mut self, dimensions: Vec2) -> &mut Self {
        self.dimensions = dimensions;
        return self;
    }

    /// Overrides the mesh that the wormhole entity uses.
    #[inline]
    pub fn with_mesh(&mut self, mesh: Handle<Mesh>) -> &mut Self {
        self.mesh_asset = Some(mesh);
        return self;
    }

    /// Sets the resolution of the generated image.
    /// Irrelevant if `with_image` is used. Defaults to the screen resolution.
    #[inline]
    pub fn with_resolution(&mut self, resolution: UVec2) -> &mut Self {
        self.resolution = resolution;
        return self;
    }

    /// Uses `image` as the render texture, rather than creating a new image.
    #[inline]
    pub fn with_image(&mut self, image: Handle<Image>) -> &mut Self {
        self.rend_image = Some(image);
        return self;
    }

    /// Sets a 'mask' image that will be used to define the opacity of the wormhole surface.
    /// Your [`TextureFormat`][TextureFormat] must be a [depth or stencil format][is_depth_stencil_format].
    /// 
    /// [TextureFormat]: bevy::render::render_resource::TextureFormat
    /// [is_depth_stencil_format]: bevy::render::render_resource::TextureFormat::is_depth_stencil_format
    #[inline]
    pub fn with_mask(&mut self, mask: Handle<Image>) -> &mut Self {
        self.mask_image = Some(mask);
        return self;
    }
}

pub struct WormholeCameraConfig {
    transform: Transform,
    render_order: isize,
}

impl WormholeCameraConfig {
    // Default, but private.
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            render_order: -1,
        }
    }

    /// Sets the [`Transform`] of the camera.
    /// Defaults to [`Transform::default()`].
    #[inline]
    pub fn with_transform(&mut self, transform: Transform) -> &mut Self {
        self.transform = transform;
        return self;
    }

    /// Sets the order at which the camera renders.
    /// See [`Camera::order`] for more information.
    /// Defaults to `-1`.
    #[inline]
    pub fn with_render_order(&mut self, order: isize) -> &mut Self {
        self.render_order = order;
        return self;
    }
}

/// Data required to call [`build`](WormholeBuilder::build) on a [`WormholeBuilder`].
#[allow(missing_docs)]
pub struct WormholeBuilderContext<'a, 'w, 's> {
    pub commands: &'a mut Commands<'w, 's>,
    pub meshes: &'a mut Assets<Mesh>,
    pub images: &'a mut Assets<Image>,
    pub shaders: &'a mut Assets<WormholeShader>,
}

/// Output by [`WormholeBuilder`].
pub struct BuiltWormholeData {
    /// The ID of the wormhole camera.
    pub camera_id: Entity,
    /// A weak handle to the image the camera draws to.
    pub camera_image: AssetId<Image>,

    /// The ID of the orange wormhole entity.
    pub orange_id: Entity,
    /// A weak handle to the mesh the orange wormhole uses.
    pub orange_mesh: AssetId<Mesh>,

    /// The ID of the blue wormhole entity.
    pub blue_id: Entity,
    /// A weak handle to the mesh the blue wormhole uses.
    pub blue_mesh: AssetId<Mesh>,
}

fn build_wormholes(
    builder: WormholeBuilder,
    context: WormholeBuilderContext,
) -> Result<BuiltWormholeData, ()> {
    // Generate ids early since entities reference eachother
    let camera_id = context.commands.spawn_empty().id();
    let orange_id = context.commands.spawn_empty().id();
    let blue_id = context.commands.spawn_empty().id();

    let render_image = builder.blue.rend_image.clone().unwrap_or_else(|| build_target_image(context.images, builder.blue.resolution));

    // Attach components to the camera entity
    context.commands.entity(camera_id).insert((
        Camera3dBundle {
            camera: Camera {
                target: RenderTarget::Image(render_image.clone()),
                order: builder.camera.render_order,
                ..default()
            },
            ..default()
        },
        WormholeCamera {
            wormholes: [orange_id, blue_id],
        },
    ));

    // Attach components to the orange portal
    let orange_mesh = build_mesh(context.meshes, &builder.orange);
    context.commands.entity(orange_id).insert((
        MaterialMeshBundle {
            transform: builder.orange.transform,
            mesh: orange_mesh.clone(),
            material: context.shaders.add(WormholeShader {
                texture: render_image.clone(),
                stencil: builder.orange.mask_image,
            }),
            ..default()
        },
        NotShadowCaster,
        Wormhole {
            counterpart: blue_id,
        },
    ));

    // Attach components to the blue portal
    let blue_mesh = build_mesh(context.meshes, &builder.blue);
    context.commands.entity(blue_id).insert((
        MaterialMeshBundle {
            transform: builder.blue.transform,
            mesh: blue_mesh.clone(),
            material: context.shaders.add(WormholeShader {
                texture: render_image.clone(),
                stencil: builder.blue.mask_image,
            }),
            ..default()
        },
        NotShadowCaster,
        Wormhole {
            counterpart: orange_id
        },
    ));

    Ok(BuiltWormholeData {
        camera_id,
        camera_image: render_image.clone().into(),
        orange_id,
        orange_mesh: orange_mesh.into(),
        blue_id,
        blue_mesh: blue_mesh.into(),
    })
}

fn build_target_image(
    images: &mut Assets<Image>,
    resolution: UVec2,
) -> Handle<Image> {
    let size = Extent3d {
        width: resolution.x,
        height: resolution.y,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            usage: WORMHOLE_TEXTURE_USAGES,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);

    images.add(image)
}

fn build_mesh(
    meshes: &mut Assets<Mesh>,
    wormhole: &WormholeSideConfig,
) -> Handle<Mesh> {
    if let Some(mesh) = &wormhole.mesh_asset {
        return mesh.clone();
    } else {
        let plane = Plane3d::new(Vec3::Y).mesh().size(wormhole.dimensions.x, wormhole.dimensions.y);
        return meshes.add(plane);
    }
}