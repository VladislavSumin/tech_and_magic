mod network_plugin;

use bevy::prelude::*;
use crate::network_plugin::NetworkPlugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(NetworkPlugin)
        .run();
}
