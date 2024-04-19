use bevy::prelude::*;
use bevy::ecs::query::QueryItem;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::binding_types::{sampler, texture_2d, uniform_buffer};
use bevy::render::render_resource::{BindGroupLayout, BindGroupLayoutEntries, SamplerBindingType, ShaderStages, TextureSampleType};
use bevy::render::renderer::RenderDevice;
use bevy::render::{render_graph::*, renderer::RenderContext, view::ViewTarget};

#[derive(Default)]
pub(crate) struct WormholeRenderNode;

impl ViewNode for WormholeRenderNode {
    type ViewQuery = (
        &'static ViewTarget,
    );

    fn run<'w>(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        view_query: QueryItem<'w, Self::ViewQuery>,
        world: &'w bevy::prelude::World,
    ) -> Result<(), NodeRunError> {
        todo!()
    }
}

pub(crate) struct WormholePipeline;

impl FromWorld for WormholePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "wormholes_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<WormholeSettings>(false),
                )
            )
        );

        Self
    }
}

#[derive(Clone, Copy, Default, Component, ExtractComponent)]
pub(crate) struct WormholeSettings {

}