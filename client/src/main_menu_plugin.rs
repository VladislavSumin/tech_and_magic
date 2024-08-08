use bevy::prelude::*;
use crate::game_state::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), spawn)
            .add_systems(OnExit(GameState::MainMenu), despawn)
        ;
    }
}


#[derive(Component)]
struct MainMenu;

fn spawn(
    mut commands: Commands,
) {
    commands.spawn((
        TextBundle::from_section("Main menu", TextStyle::default())
            .with_style(Style {
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            }),
        MainMenu,
    ));
}

fn despawn(
    mut commands: Commands,
    text_query: Query<Entity, With<MainMenu>>,
) {
    let text = text_query.single();
    commands.entity(text).despawn();
}
