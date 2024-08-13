//! # Server players
//! Создает и обновляет список подключенных игроков [Players]. Для каждого игрока создает [Entity] и управляет ее
//! жизненным циклом, на эти сущности можно вешать дополнительные компоненты.

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_renet::renet::{ClientId, RenetServer, ServerEvent};
use core_loading::LoadingState;
use core_network_server::ChannelMapping;
use core_players_shared::{PLAYERS_SYNC, PlayerStateUpdate, SharedPlayersPlugin};

pub struct ServerPlayersPlugin;

impl Plugin for ServerPlayersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SharedPlayersPlugin)
            .init_resource::<Players>()
            .add_systems(Update, handle_server_events.run_if(in_state(LoadingState::Loaded)))
        ;
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Players(HashMap<ClientId, Entity>);

/// Обрабатывает события [ServerEvent] и управляет наполнением [Players].
fn handle_server_events(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut players: ResMut<Players>,
    mut renet_server: ResMut<RenetServer>,
    channel_mapping: Res<ChannelMapping>,
) {
    for server_event in server_events.read() {
        let channel_id = channel_mapping.server_channels.get(PLAYERS_SYNC).unwrap();
        match server_event {
            ServerEvent::ClientConnected { client_id } => {
                debug!("Player {} connected", client_id);

                for (current_client_id, _) in players.iter() {
                    let message = PlayerStateUpdate::PlayerConnected { client_id: *current_client_id };
                    let message = bincode::serialize(&message).unwrap();
                    renet_server.send_message(*client_id, *channel_id, message);
                }

                let id = commands.spawn(()).id();
                players.insert(*client_id, id);
                let broadcast_message = PlayerStateUpdate::PlayerConnected { client_id: *client_id };
                let broadcast_message = bincode::serialize(&broadcast_message).unwrap();
                renet_server.broadcast_message(*channel_id, broadcast_message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                debug!("Player {} disconnected, reason {}", client_id, reason);
                let id = players.remove(client_id).unwrap();
                commands.entity(id).despawn();
                let broadcast_message = PlayerStateUpdate::PlayerDisconnected { client_id: *client_id };
                let broadcast_message = bincode::serialize(&broadcast_message).unwrap();
                renet_server.broadcast_message(*channel_id, broadcast_message);
            }
        }
    }
}