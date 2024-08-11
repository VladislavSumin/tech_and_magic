use bevy::prelude::*;
use feature_game_state::GameState;

/// Плагин создает тестовый куб.
pub struct TestCubePlugin;

impl Plugin for TestCubePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), spawn)
            .add_systems(OnExit(GameState::InGame), despawn)
        ;
    }
}

#[derive(Component)]
struct TestCube;

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        TestCube
    ));
}

fn despawn(
    mut commands: Commands,
    test_cube_query: Query<Entity, With<TestCube>>,
) {
    let cube = test_cube_query.single();
    commands.entity(cube).despawn();
}