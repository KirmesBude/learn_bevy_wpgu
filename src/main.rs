use bevy::prelude::*;
use learn_bevy_wgpu::LearnWgpuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LearnWgpuPlugin)
        .run();
}
