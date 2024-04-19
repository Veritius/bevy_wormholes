use bevy::prelude::*;

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
        // Generate ids early since entities reference eachother
        let camera_id = context.commands.spawn_empty().id();
        let orange_id = context.commands.spawn_empty().id();
        let blue_id = context.commands.spawn_empty().id();

        todo!()
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
}

impl WormholeSideConfig {
    // Default, but private.
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            dimensions: Vec2::splat(1.0),
            resolution: UVec2::splat(128),
        }
    }

    #[inline]
    pub fn with_transform(&mut self, transform: Transform) -> &mut Self {
        self.transform = transform;
        return self;
    }

    #[inline]
    pub fn with_dimensions(&mut self, dimensions: Vec2) -> &mut Self {
        self.dimensions = dimensions;
        return self;
    }

    #[inline]
    pub fn with_resolution(&mut self, resolution: UVec2) -> &mut Self {
        self.resolution = resolution;
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

    #[inline]
    pub fn with_transform(&mut self, transform: Transform) -> &mut Self {
        self.transform = transform;
        return self;
    }

    #[inline]
    pub fn with_render_order(&mut self, order: isize) -> &mut Self {
        self.render_order = order;
        return self;
    }
}

/// Data required to call [`build`](WormholeBuilder::build) on a [`WormholeBuilder`].
pub struct WormholeBuilderContext<'a, 'w, 's> {
    pub commands: &'a mut Commands<'w, 's>,
    pub meshes: &'a mut Assets<Mesh>,
    pub images: &'a mut Assets<Image>,
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