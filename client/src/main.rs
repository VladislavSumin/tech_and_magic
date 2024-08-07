mod render_fps_plugin;
mod loading_plugin;
mod camera_plugin;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use crate::camera_plugin::CameraPlugin;
use crate::loading_plugin::LoadingPlugin;
use crate::render_fps_plugin::RenderFpsPlugin;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)

        // Project plugins
        .add_plugins((
            RenderFpsPlugin,
            LoadingPlugin,
            CameraPlugin,
        ))

        .run();
}

