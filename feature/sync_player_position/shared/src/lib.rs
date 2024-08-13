use bevy::prelude::*;
use bevy_renet::renet::SendType;
use core_network_shared::app_channel_registrar::AppChannelRegistrar;
use core_network_shared::channel_registration::ChannelRegistrationInfo;

pub struct SyncPlayerPositionSharedPlugin;
pub const CLIENT_PLAYER_POSITION: &str = "client_player_position";

impl Plugin for SyncPlayerPositionSharedPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_client_channel(
                CLIENT_PLAYER_POSITION,
                ChannelRegistrationInfo {
                    send_type: SendType::Unreliable,
                    max_memory_usage_bytes: 8 * 1024,
                },
            )
        ;
    }
}