//! # Server players
//! Создает и обновляет список подключенных игроков [Players]. Для каждого игрока создает [Entity] и управляет ее
//! жизненным циклом, на эти сущности можно вешать дополнительные компоненты.

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_renet::renet::{ClientId, ServerEvent};
use core_loading::LoadingState;
use core_players_shared::SharedPlayersPlugin;

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
) {
    for server_event in server_events.read() {
        match server_event {
            ServerEvent::ClientConnected { client_id } => {
                debug!("Player {} connected", client_id);
                let id = commands.spawn(()).id();
                players.insert(*client_id, id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                debug!("Player {} disconnected, reason {}", client_id, reason);
                let id = players.remove(client_id).unwrap();
                commands.entity(id).despawn();
            }
        }
    }
}