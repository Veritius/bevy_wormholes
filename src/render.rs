//! Advanced tools for control over rendering.

use bevy::prelude::*;
use bevy::render::extract_component::{ExtractComponent, ExtractComponentPlugin};
use bevy::render::render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext, RenderLabel};
use bevy::render::render_resource::{BindGroupLayout, CachedRenderPipelineId};
use bevy::render::renderer::RenderContext;
use bevy::render::RenderApp;

pub(super) fn setup_rendering(app: &mut App) {
    app.add_plugins(ExtractComponentPlugin::<WormholeShader>::default());

    let render_app = app.sub_app_mut(RenderApp);

    let mut graph = render_app.world.resource_mut::<RenderGraph>();
    graph.add_node(WormholeRenderLabel, WormholeRenderNode);
}

/// [`RenderLabel`] for wormhole rendering.
#[derive(Debug, Clone, PartialEq, Eq, Hash, RenderLabel)]
pub struct WormholeRenderLabel;

/// Rendering [`Node`] for wormhole rendering.
pub struct WormholeRenderNode;

impl Node for WormholeRenderNode {
    fn run<'w>(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        todo!()
    }
}

#[derive(Resource)]
pub(crate) struct WormholeShaderData {
    pub pipeline_id: CachedRenderPipelineId,
    pub uniform_layout: BindGroupLayout,
}

impl FromWorld for WormholeShaderData {
    fn from_world(world: &mut World) -> Self {
        todo!()
    }
}

/// The shader used to draw wormholes.
#[derive(Debug, Clone, Component, ExtractComponent, Reflect)]
#[reflect(Debug, Component)]
pub struct WormholeShader {
    /// The texture that the [`WormholeCamera`](crate::WormholeCamera) renders to.
    pub texture: Handle<Image>,

    /// The stencil texture that masks off parts of the wormhole surface.
    /// `1.0` means fully visible, `0.0` means fully transparent.
    pub stencil: Option<Handle<Image>>,
}