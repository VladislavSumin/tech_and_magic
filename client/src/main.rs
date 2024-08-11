mod render_fps_plugin;
mod loading_plugin;
mod camera_plugin;
mod main_menu_plugin;
mod connecting_plugin;
mod connection_error_plugin;
mod connected_plugin;
mod test_cube_plugin;
mod key_binding_plugin;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use core_log::create_log_plugin;
use feature_game_state::GameStatePlugin;
use feature_network_client::ClientNetworkPlugin;
use crate::camera_plugin::CameraPlugin;
use crate::connected_plugin::ConnectedPlugin;
use crate::connecting_plugin::ConnectingPlugin;
use crate::connection_error_plugin::ConnectionErrorPlugin;
use crate::key_binding_plugin::KeyBindingsPlugin;
use crate::loading_plugin::LoadingPlugin;
use crate::main_menu_plugin::MainMenuPlugin;
use crate::render_fps_plugin::RenderFpsPlugin;
use crate::test_cube_plugin::TestCubePlugin;

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
            KeyBindingsPlugin,
            ClientNetworkPlugin,
            GameStatePlugin,
            MainMenuPlugin,
            ConnectingPlugin,
            ConnectionErrorPlugin,
            ConnectedPlugin,
            CameraPlugin,
            TestCubePlugin,
        ))

        .run();
}