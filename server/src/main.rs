use std::time::Duration;
use bevy::app::ScheduleRunnerPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use core_log::create_log_plugin;
use core_network_server::ServerNetworkPlugin;
use feature_sync_player_position_server::SyncPlayerPositionPlugin;

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins
                // Попытка сделать стабильный TPS на сервере, к сожалению это работает плохо.
                // TODO придумать другую систему для контроля TPS.
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0)))
        )
        .add_plugins(create_log_plugin())
        .add_plugins((
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default()
        ))
        .add_plugins((
            ServerNetworkPlugin,
            SyncPlayerPositionPlugin,
        ))
        .run();
}
