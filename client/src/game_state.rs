use bevy::prelude::*;
use crate::loading_plugin::LoadingState;

/// Плагин отвечающий за основное состояние игры.
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_sub_state::<GameState>()
        ;
    }
}
/// Основное состояние игры
#[derive(SubStates, Clone, Hash, Debug, PartialEq, Eq, Default)]
#[source(LoadingState = LoadingState::Loaded)]
pub enum GameState {
    #[default]
    MainMenu,
}