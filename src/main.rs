use bevy::prelude::*;
use learn_bevy_wgpu::{CameraLearnWgpuBundle, LearnWgpuPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LearnWgpuPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(CameraLearnWgpuBundle::default());
}
