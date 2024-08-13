use bevy::prelude::*;
use bevy_renet::renet::{RenetClient, RenetServer};
use core_loading::LoadingState;
use core_network_server::ChannelMapping;
use feature_sync_player_position_shared::{CLIENT_PLAYER_POSITION, ClientPlayerPosition, SyncPlayerPositionSharedPlugin};

pub struct SyncPlayerPositionPlugin;

impl Plugin for SyncPlayerPositionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SyncPlayerPositionSharedPlugin)
            .add_systems(Update, receive_player_position.run_if(in_state(LoadingState::Loaded)))
        ;
    }
}

fn receive_player_position(
    channel_mapping: Res<ChannelMapping>,
    mut server: ResMut<RenetServer>,
) {
    let channel_id = channel_mapping.client_channels.get(CLIENT_PLAYER_POSITION).unwrap();
    for client_id in server.clients_id() {
        while let Some(data) = server.receive_message(client_id, *channel_id) {
            let data: ClientPlayerPosition = bincode::deserialize(&data).unwrap();
            info!("data = {}", data.0);
        }
    }
}