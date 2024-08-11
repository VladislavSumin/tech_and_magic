mod render_fps_plugin;
mod loading_plugin;
mod camera_plugin;
mod main_menu_plugin;
mod game_state;
mod connecting_plugin;
mod connection_error_plugin;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use core_log::create_log_plugin;
use feature_network_client::ClientNetworkPlugin;
use crate::camera_plugin::CameraPlugin;
use crate::connecting_plugin::ConnectingPlugin;
use crate::connection_error_plugin::ConnectionErrorPlugin;
use crate::game_state::GameStatePlugin;
use crate::loading_plugin::LoadingPlugin;
use crate::main_menu_plugin::MainMenuPlugin;
use crate::render_fps_plugin::RenderFpsPlugin;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(
            DefaultPlugins
                .set(create_log_plugin())
        )
        .add_plugins(FrameTimeDiagnosticsPlugin)

        // 3pc plugins
        .add_plugins(EguiPlugin)

        // Project plugins
        .add_plugins((
            RenderFpsPlugin,
            LoadingPlugin,
            ClientNetworkPlugin,
            GameStatePlugin,
            MainMenuPlugin,
            ConnectingPlugin,
            ConnectionErrorPlugin,
            CameraPlugin,
        ))

        .run();
}