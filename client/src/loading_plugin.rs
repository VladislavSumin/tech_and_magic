use bevy::prelude::*;

/// Выводит фейковый прогресс загрузки в состоянии [LoadingState::Loading], через
/// какое-то время переводит в состояние [LoadingState::Loaded] и убирает текст.
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<LoadingState>()
            .add_systems(OnEnter(LoadingState::Loading), spawn_loading_text)
            .add_systems(Update, update_loading_text.run_if(in_state(LoadingState::Loading)))
            .add_systems(OnExit(LoadingState::Loading), despawn_loading_text)
        ;
    }
}

#[derive(States, Hash, PartialEq, Eq, Default, Clone, Debug)]
pub enum LoadingState {
    #[default]
    Loading,
    Loaded,
}


#[derive(Component)]
struct LoadingText;
fn spawn_loading_text(
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

fn update_loading_text(
    mut progress: Local<f32>,
    mut text_query: Query<&mut Text, With<LoadingText>>,
    mut next_loading_state: ResMut<NextState<LoadingState>>,
) {
    let mut text = text_query.single_mut();
    *progress += 1.0;

    if *progress >= 100.0 {
        next_loading_state.set(LoadingState::Loaded)
    }

    text.sections[0].value = format!("Loading...: {:.1}%", *progress);
}

fn despawn_loading_text(
    mut commands: Commands,
    text_query: Query<Entity, With<LoadingText>>,
) {
    let text = text_query.single();
    commands.entity(text).despawn();
}