mod render_fps_plugin;
mod loading_plugin;
mod camera_plugin;
mod main_menu_plugin;
mod game_state;
mod network_plugin;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use core_log::create_log_plugin;
use crate::camera_plugin::CameraPlugin;
use crate::game_state::GameStatePlugin;
use crate::loading_plugin::LoadingPlugin;
use crate::main_menu_plugin::MainMenuPlugin;
use crate::network_plugin::NetworkPlugin;
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
            NetworkPlugin,
            GameStatePlugin,
            MainMenuPlugin,
            CameraPlugin,
        ))

        .run();
}