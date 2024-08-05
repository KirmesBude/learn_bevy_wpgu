use bevy::{
    app::Plugin,
    prelude::*,
    render::{
        camera::{CameraMainTextureUsages, CameraRenderGraph},
        view::VisibleEntities,
    },
};
use render::{graph::CoreLearnWgpu, LearnWgpuRenderPlugin};

pub mod render;

pub struct LearnWgpuPlugin;

impl Plugin for LearnWgpuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(LearnWgpuRenderPlugin);
    }
}

#[derive(Default, Component, Clone)]
pub struct CameraLearnWgpu;

/* TODO: How much of this do I need? */
#[derive(Bundle, Clone)]
pub struct CameraLearnWgpuBundle {
    pub camera: Camera,
    pub camera_render_graph: CameraRenderGraph,
    pub projection: Projection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub camera_learn_wgpu: CameraLearnWgpu,
    pub main_texture_usages: CameraMainTextureUsages,
}

impl Default for CameraLearnWgpuBundle {
    fn default() -> Self {
        Self {
            camera: Default::default(),
            camera_render_graph: CameraRenderGraph::new(CoreLearnWgpu),
            projection: Default::default(),
            visible_entities: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            camera_learn_wgpu: Default::default(),
            main_texture_usages: Default::default(),
        }
    }
}
