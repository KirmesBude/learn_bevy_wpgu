use bevy::prelude::*;
use tutorial1_graph_node::Tutorial1GraphNodePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) /* TODO: Reduce DefaultPlugins we include */
        .add_plugins(Tutorial1GraphNodePlugin)
        .run();
}
