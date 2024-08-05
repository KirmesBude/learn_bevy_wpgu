use bevy::app::Plugin;
use render::LearnWgpuRenderPlugin;

mod render;

pub struct LearnWgpuPlugin;

impl Plugin for LearnWgpuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(LearnWgpuRenderPlugin);
    }
}
