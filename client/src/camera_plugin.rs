use bevy::math::vec3;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_player_camera)
        ;
    }
}

/// Создает и настраивает дефолтную камеру игрока
fn setup_player_camera(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 0., 128.0)
                .looking_at(vec3(14., 14., 0.), Vec3::Z),
            ..default()
        },
    ));
}