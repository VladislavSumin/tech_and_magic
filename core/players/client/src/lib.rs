//! # Client players
//! Создает и обновляет список подключенных игроков [Players]. Для каждого игрока создает [Entity] и управляет ее
//! жизненным циклом, на эти сущности можно вешать дополнительные компоненты.
//! **Внимание** текущий игрок тоже будет представлен в этом списке.
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_renet::renet::{Bytes, ClientId, RenetClient, RenetServer};
use core_network_client::ChannelMapping;
use core_players_shared::{PLAYERS_SYNC, PlayerStateUpdate, SharedPlayersPlugin};

pub struct ClientPlayersPlugin;

impl Plugin for ClientPlayersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SharedPlayersPlugin)
            .init_resource::<Players>()
            .add_systems(Update, handle_server_updates.run_if(resource_exists::<RenetClient>))
            .add_systems(Update, clean_players_on_disconnect.run_if(resource_removed::<RenetClient>()))
        ;
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Players(HashMap<ClientId, Entity>);

fn clean_players_on_disconnect(
    mut players: ResMut<Players>,
) {
    players.clear();
}

fn handle_server_updates(
    mut client: ResMut<RenetClient>,
    channel_mapping: Res<ChannelMapping>,
    mut players: ResMut<Players>,
    mut commands: Commands,
) {
    let channel_id = channel_mapping.server_channels.get(PLAYERS_SYNC).unwrap();
    while let Some(message) = client.receive_message(*channel_id) {
        let message: PlayerStateUpdate = bincode::deserialize(&message).unwrap();
        match message {
            PlayerStateUpdate::PlayerConnected { client_id } => {
                debug!("Player {client_id} connected");
                let id = commands.spawn(()).id();
                players.insert(client_id, id);
            }
            PlayerStateUpdate::PlayerDisconnected { client_id } => {
                debug!("Player {client_id} disconnected");
                let id = players.remove(&client_id).unwrap();
                commands.entity(id).despawn();
            }
        }
    }
}