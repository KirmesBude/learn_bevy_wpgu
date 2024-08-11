use bevy::prelude::*;
use tutorial3_buffer::Tutorial3BufferPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Tutorial3BufferPlugin)
        .run();
}
