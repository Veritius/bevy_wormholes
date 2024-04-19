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

    /// Applies the `Builder`.
    pub fn apply(self, commands: &mut Commands) -> Result<BuiltWormholeEntities, ()> {
        // Generate ids early since entities reference eachother
        let camera_id = commands.spawn_empty().id();
        let orange_id = commands.spawn_empty().id();
        let blue_id = commands.spawn_empty().id();

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
}

impl WormholeSideConfig {
    // Default, but private.
    fn default() -> Self {
        Self {
            transform: Transform::default(),
        }
    }

    pub fn with_transform(&mut self, transform: Transform) -> &mut Self {
        self.transform = transform;
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

    pub fn with_transform(&mut self, transform: Transform) -> &mut Self {
        self.transform = transform;
        return self;
    }

    pub fn with_render_order(&mut self, order: isize) -> &mut Self {
        self.render_order = order;
        return self;
    }
}

/// Output by [`WormholeBuilder`].
pub struct BuiltWormholeEntities {
    /// The ID of the wormhole camera.
    pub camera_id: Entity,
    /// The ID of the orange wormhole entity.
    pub orange_id: Entity,
    /// The ID of the blue wormhole entity.
    pub blue_id:   Entity,
}