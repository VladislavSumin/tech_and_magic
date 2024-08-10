use bevy::prelude::*;
use core_log::create_log_plugin;
use feature_network_server::ServerNetworkPlugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(create_log_plugin())
        .add_plugins(ServerNetworkPlugin)
        .run();
}
