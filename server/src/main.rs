mod network_plugin;

use bevy::prelude::*;
use core_log::create_log_plugin;
use crate::network_plugin::NetworkPlugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(create_log_plugin())
        .add_plugins(NetworkPlugin)
        .run();
}
