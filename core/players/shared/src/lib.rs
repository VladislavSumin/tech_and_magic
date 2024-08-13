//! # Shared players
//! Создает каналы для синхронизации игроков между клиентом и сервером.
use std::time::Duration;
use bevy::prelude::*;
use bevy_renet::renet::{ClientId, SendType};
use serde::{Deserialize, Serialize};
use core_network_shared::app_channel_registrar::AppChannelRegistrar;
use core_network_shared::channel_registration::ChannelRegistrationInfo;

pub struct SharedPlayersPlugin;
pub const PLAYERS_SYNC: &str = "players_sync";

#[derive(Serialize, Deserialize)]
pub enum PlayerStateUpdate {
    PlayerConnected { client_id: ClientId },
    PlayerDisconnected { client_id: ClientId },
}

impl Plugin for SharedPlayersPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_server_channel(
                PLAYERS_SYNC,
                ChannelRegistrationInfo {
                    send_type: SendType::ReliableOrdered { resend_time: Duration::from_millis(500) },
                    max_memory_usage_bytes: 8 * 1024, // TODO рассчитать размеры.
                },
            )
        ;
    }
}