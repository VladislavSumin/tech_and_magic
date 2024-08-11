use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use feature_game_state::GameState;

/// Переводит состояние игры в [GameState::InGame] при успешном подключении к серверу.
pub struct ConnectedPlugin;

impl Plugin for ConnectedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                handle_connected_state
                    .run_if(resource_exists::<RenetClient>)
                    .run_if(in_state(GameState::ConnectingToServer)),
            )
        ;
    }
}

fn handle_connected_state(
    renet_client: Res<RenetClient>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if renet_client.is_connected() {
        game_state.set(GameState::InGame);
    }
}