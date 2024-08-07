use bevy::{
    prelude::*,
    render::{
        render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext},
        render_phase::TrackedRenderPass,
        render_resource::{
            CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
            RenderPassDescriptor, StoreOp,
        },
        renderer::RenderContext,
        view::ExtractedWindows,
        RenderApp,
    },
};

pub struct LearnWgpuRenderPlugin;

impl Plugin for LearnWgpuRenderPlugin {
    fn build(&self, app: &mut App) {
        let render_app = match app.get_sub_app_mut(RenderApp) {
            Some(render_app) => render_app,
            None => return,
        };

        let main_pass_node = MainPassNode;
        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(graph::NodeLearnWgpu::Main, main_pass_node);
    }
}

pub mod graph {
    use bevy::render::render_graph::RenderLabel;

    #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
    pub enum NodeLearnWgpu {
        Main,
    }
}

#[derive(Default)]
pub struct MainPassNode;
impl Node for MainPassNode {
    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        let windows = world.get_resource::<ExtractedWindows>().unwrap();
        let view = windows
            .windows
            .get(&windows.primary.unwrap())
            .unwrap()
            .swap_chain_texture_view
            .as_ref()
            .unwrap(); /* TODO: This needs to be cleaned up */

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
                    view,
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
