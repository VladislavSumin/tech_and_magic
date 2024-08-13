use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use core_network_client::ChannelMapping;
use feature_game_state::GameState;
use feature_sync_player_position_shared::{CLIENT_PLAYER_POSITION, ClientPlayerPosition, SyncPlayerPositionSharedPlugin};

pub struct SyncPlayerPositionPlugin;

/// Маркер игрока. Координаты сущности с таким маркером будут передаваться на сервер.
#[derive(Component)]
pub struct Player;

impl Plugin for SyncPlayerPositionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SyncPlayerPositionSharedPlugin)
            .add_systems(Update, send_player_position.run_if(in_state(GameState::InGame)))
        ;
    }
}

fn send_player_position(
    player_transform_query: Query<&Transform, With<Player>>,
    channel_mapping: Res<ChannelMapping>,
    mut client: ResMut<RenetClient>,
) {
    let player_transform = player_transform_query.single();
    let id = channel_mapping.client_channels.get(CLIENT_PLAYER_POSITION).unwrap();
    let message = ClientPlayerPosition(player_transform.translation);
    let message = bincode::serialize(&message).unwrap();
    client.send_message(*id, message);
}