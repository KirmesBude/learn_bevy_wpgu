use bevy::prelude::*;
use tutorial4_textures::Tutorial4Textures;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Tutorial4Textures)
        .run();
}
