use bevy::math::vec3;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_player_camera)
        .run();
}

/// Маркер для дефолтной камеры игрока
#[derive(Component)]
pub struct PlayerCamera;

/// Создает и настраивает дефолтную камеру игрока
fn setup_player_camera(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 0., 128.0)
                .looking_at(vec3(14., 14., 0.), Vec3::Z),
            ..default()
        },
        PlayerCamera,
    ));
}
