use std::time::Duration;
use bevy::app::ScheduleRunnerPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use core_loading::LoadingPlugin;
use core_log::create_log_plugin;
use core_network_server::ServerNetworkPlugin;
use core_players_server::ServerPlayersPlugin;
use feature_sync_player_position_server::SyncPlayerPositionPlugin;

fn main() {
    App::new()
        .add_plugins((
            // Bevy
            MinimalPlugins
                // Попытка сделать стабильный TPS на сервере, к сожалению это работает плохо.
                // TODO придумать другую систему для контроля TPS.
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0))),
            create_log_plugin(),
            StatesPlugin,
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),

            // Custom
            LoadingPlugin,
            ServerNetworkPlugin,
            ServerPlayersPlugin,
            SyncPlayerPositionPlugin,
        ))
        .run();
}
