use bevy::prelude::*;
use core_loading::{LoadingProgress, LoadingState};

/// Отображает текущий прогресс загрузки.
pub struct LoadingUiPlugin;

impl Plugin for LoadingUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(LoadingState::Loading), spawn)
            .add_systems(Update, update.run_if(in_state(LoadingState::Loading)))
            .add_systems(OnExit(LoadingState::Loading), despawn)
        ;
    }
}

#[derive(Component)]
struct LoadingText;
fn spawn(
    mut commands: Commands,
) {
    commands.spawn((
        TextBundle::from_section("Loading...", TextStyle::default())
            .with_style(Style {
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            }),
        LoadingText,
    ));
}

fn update(
    progress: Res<LoadingProgress>,
    mut text_query: Query<&mut Text, With<LoadingText>>,
) {
    let mut text = text_query.single_mut();
    text.sections[0].value = format!("Loading...: {:.1}%", **progress * 100.);
}

fn despawn(
    mut commands: Commands,
    text_query: Query<Entity, With<LoadingText>>,
) {
    let text = text_query.single();
    commands.entity(text).despawn();
}