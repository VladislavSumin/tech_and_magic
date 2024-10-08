use std::f32::consts::PI;
use bevy::input::mouse::MouseMotion;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use feature_game_state::GameState;
use feature_sync_player_position_client::Player;
use crate::key_binding_plugin::KeyBindings;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MovementSettings>()
            .add_systems(Startup, setup_player_camera)
            .add_systems(Update, player_move.run_if(in_state(GameState::InGame)))
            .add_systems(Update, player_look.run_if(in_state(GameState::InGame)))
            .add_systems(Update, cursor_grab.run_if(in_state(GameState::InGame)))
        ;
    }
}

/// Настройки перемещения камеры
#[derive(Resource)]
pub struct MovementSettings {
    /// Чувствительность мыши
    pub sensitivity: f32,
    /// Скорость перемещения камеры
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.0002,
            speed: 32.,
        }
    }
}

#[derive(Component)]
struct PlayerCamera;

/// Создает и настраивает дефолтную камеру игрока
fn setup_player_camera(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 0., 128.0)
                .looking_at(vec3(14., 14., 0.), Vec3::Z),
            ..default()
        },
        Player,
        PlayerCamera,
    ));
}

/// Handles keyboard input and movement
fn player_move(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<KeyBindings>,
    mut query: Query<(&PlayerCamera, &mut Transform)>, //    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    let window = primary_window.single();
    for (_camera, mut transform) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;

        let local_y = transform.local_x();
        let forward = -Vec3::new(local_y.y, -local_y.x, 0.);
        let right = Vec3::new(local_y.x, local_y.y, 0.);

        for key in keys.get_pressed() {
            match window.cursor.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    let key = *key;
                    if key == key_bindings.move_forward {
                        velocity += forward;
                    } else if key == key_bindings.move_backward {
                        velocity -= forward;
                    } else if key == key_bindings.move_left {
                        velocity -= right;
                    } else if key == key_bindings.move_right {
                        velocity += right;
                    } else if key == key_bindings.move_up {
                        velocity += Vec3::Z;
                    } else if key == key_bindings.move_down {
                        velocity -= Vec3::Z;
                    }
                }
            }

            velocity = velocity.normalize_or_zero();

            transform.translation += velocity * time.delta_seconds() * settings.speed
        }
    }
}

/// Handles looking around if cursor is locked
fn player_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut motion: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<PlayerCamera>>,
) {
    let window = primary_window.single();
    for mut transform in query.iter_mut() {
        for ev in motion.read() {
            // let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            let (mut yaw, _, mut pitch) = transform.rotation.to_euler(EulerRot::ZYX);
            match window.cursor.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                    let window_scale = window.height().min(window.width());
                    pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                    yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                }
            }

            pitch = pitch.clamp(0.02 * PI, 0.98 * PI);

            // Order is important to prevent unintended roll
            transform.rotation =
                Quat::from_axis_angle(Vec3::Z, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    }
}

fn cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = primary_window.single_mut();
    if keys.just_pressed(key_bindings.toggle_grab_cursor) {
        toggle_grab_cursor(&mut window);
    }
}

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}