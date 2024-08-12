use bevy::prelude::*;
use core_loading::LoadingState;

/// Плагин отвечающий за создание [GameState].
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_sub_state::<GameState>()
        ;
    }
}

/// Основное состояние игры.
#[derive(SubStates, Clone, Hash, Debug, PartialEq, Eq, Default)]
#[source(LoadingState = LoadingState::Loaded)]
pub enum GameState {
    /// Отображается главное меню.
    #[default]
    MainMenu,
    /// Происходит подключение к серверу, отображается плашка с прогрессом подключения.
    ConnectingToServer,
    /// Произошла ошибка соединения, это может произойти как во время подключения, так и во время игры.
    /// Отображается сообщение об ошибке соединения.
    ConnectionError,
    /// Соединение установлено и в данный момент запущена игра. Состояния самой игры находятся отдельно.
    InGame,
}
