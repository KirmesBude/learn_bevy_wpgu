use bevy::{
    prelude::*,
    render::{
        graph::CameraDriverLabel,
        render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext, RenderLabel},
        render_resource::{
            CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
            RenderPassDescriptor, StoreOp,
        },
        renderer::RenderContext,
        view::ExtractedWindows,
        RenderApp,
    },
};

// You can easily organize your code with plugins in bevy, so that is what we are doing here.
// As a plugin we can interact with App to get everything we need.
pub struct Tutorial1GraphNodePlugin;

impl Plugin for Tutorial1GraphNodePlugin {
    fn build(&self, app: &mut App) {
        // Bevy splits up your usual logic world and the render world to allow for things such as pipelined rendering.
        // Later tutorials will go into more depth on how those two interact with each other.
        // For now everything we care about happens on the RenderApp and we do not need anything from the usual logic world.
        let render_app = match app.get_sub_app_mut(RenderApp) {
            Some(render_app) => render_app,
            None => return,
        };

        // In wgpu we need to set up some stuff so the RenderPassDescriptor that we create is actually "run".
        // Bevy has built RenderGraphs on top. They are acyclic graphs that define Nodes that can connect to other Nodes.
        // A node can have inputs and outputs - for our usecase we will have a node without any of those.
        // Within a node you will usually create your RenderPassDescriptor, but you can also do other stuff here.
        // Bevy takes care of running every node in the RenderGraph.
        // In the future we will interface with RenderGraph to add sub graphs, that are driven by Cameras.
        let main_pass_node = MainPassNode;
        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(MainPassNodeLabel, main_pass_node);
        // It is necessary to ensure our Node is run after the CameraDriverNode, because it may also write to the view texture.
        render_graph.add_node_edge(CameraDriverLabel, MainPassNodeLabel);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct MainPassNodeLabel;

#[derive(Default)]
pub struct MainPassNode;
impl Node for MainPassNode {
    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        // The only thing we need right now is a texture view to write to.
        // Whereas wgpu will have preivously will previously interfaces with a windowing library to retrieve the Surface, we won't have to do any of that because bevy already did it.
        // The MainApp has extracted any windows to the RenderApp for us to use. The concept of extraction will be explored further in a future tutorial.
        let extracted_windows = world.resource::<ExtractedWindows>();
        let primary_window_entity = extracted_windows.primary.unwrap();
        let Some(view) = extracted_windows
            .get(&primary_window_entity)
            .and_then(|extracted_window| extracted_window.swap_chain_texture_view.as_ref())
        else {
            return Ok(());
        };

        // RenderContext will give us access to everything that the wgpu tutorial uses, so the following code is almost the same.
        render_context.add_command_buffer_generation_task(move |render_device| {
            // Command encoder setup
            let mut command_encoder =
                render_device.create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            {
                let _render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Render Pass"),
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
            }

            // No need to submit the command to the queue, because bevy will first batch them and queue them all together.
            // Bevy will also take care of presenting to the texture.
            command_encoder.finish()
        });

        Ok(())
    }
}
