use bevy::{
    ecs::query::QueryItem,
    prelude::*,
    render::{
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, ViewNode, ViewNodeRunner,
        },
        render_phase::TrackedRenderPass,
        render_resource::{
            CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
            RenderPassDescriptor, StoreOp,
        },
        renderer::RenderContext,
        view::ViewTarget,
        RenderApp,
    },
};
use graph::{CoreLearnWgpu, NodeLearnWgpu};

pub struct LearnWgpuRenderPlugin;

impl Plugin for LearnWgpuRenderPlugin {
    fn build(&self, app: &mut App) {
        let render_app = match app.get_sub_app_mut(RenderApp) {
            Some(render_app) => render_app,
            None => return,
        };

        render_app
            .add_render_sub_graph(CoreLearnWgpu)
            .add_render_graph_node::<ViewNodeRunner<MainPassNode>>(
                CoreLearnWgpu,
                NodeLearnWgpu::Main,
            );
    }
}

pub mod graph {
    use bevy::render::render_graph::{RenderLabel, RenderSubGraph};

    #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderSubGraph)]
    pub struct CoreLearnWgpu;

    pub mod input {
        pub const VIEW_ENTITY: &str = "view_entity";
    }

    #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
    pub enum NodeLearnWgpu {
        Main,
    }
}

#[derive(Default)]
pub struct MainPassNode;
impl ViewNode for MainPassNode {
    type ViewQuery = (&'static ViewTarget,);

    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (target,): QueryItem<'w, Self::ViewQuery>,
        _world: &'w World,
    ) -> Result<(), NodeRunError> {
        render_context.add_command_buffer_generation_task(move |render_device| {
            // Command encoder setup
            let mut command_encoder =
                render_device.create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            // Render pass setup
            let render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("main_opaque_pass_3d"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: target.get_color_attachment().view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(
                            LinearRgba {
                                red: 0.1,
                                green: 0.2,
                                blue: 0.3,
                                alpha: 1.0,
                            }
                            .into(),
                        ),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            /* This seems to be the alternative for the borrow trick */
            let mut _render_pass = TrackedRenderPass::new(&render_device, render_pass);
            drop(_render_pass);

            command_encoder.finish()
        });

        Ok(())
    }
}
