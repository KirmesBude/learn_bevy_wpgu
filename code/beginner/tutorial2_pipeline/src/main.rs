use bevy::prelude::*;
use tutorial2_pipeline::Tutorial2PipelinePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Tutorial2PipelinePlugin)
        .run();
}
