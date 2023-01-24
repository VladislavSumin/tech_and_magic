use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let log_plugin = LogPlugin {
        level: Level::DEBUG,
        filter: "bevy=info,wgpu=warn,naga=info,gilrs=info".to_string(),
    };

    let default_plugins = DefaultPlugins.set(log_plugin);

    App::new()
        .add_plugins(default_plugins)
        .add_plugin(WorldInspectorPlugin)

        .add_plugin(PlayerPlugin)

        .add_startup_system(spawn_test_cube_system)

        .run();
}

fn spawn_test_cube_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    debug!("Spawning test cube");
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { ..default() })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0., -1., 0.),
        ..default()
    });
}
