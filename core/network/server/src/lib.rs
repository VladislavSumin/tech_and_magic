use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::prelude::*;
use bevy_renet::renet::{ConnectionConfig, RenetServer, ServerEvent};
use bevy_renet::renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::RenetServerPlugin;
use bevy_renet::transport::NetcodeServerPlugin;
use core_loading::LoadingState;
use core_network_shared::channel_registration::{ChannelRegistration, NetworkChannelRegistrationPlugin};
use core_network_shared::DEFAULT_PORT;

pub use core_network_shared::channel_registration::ChannelMapping;

/// Отвечает за базовую настройку сети на сервере.
pub struct ServerNetworkPlugin;

const LOCALHOST: &str = "127.0.0.1";

impl Plugin for ServerNetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                RenetServerPlugin,
                NetcodeServerPlugin,
                NetworkChannelRegistrationPlugin,
            ))
            .add_systems(OnEnter(LoadingState::Loaded), init_network_layer)
            .add_systems(Update, handle_events.run_if(in_state(LoadingState::Loaded)))
        ;
    }
}

fn init_network_layer(
    channel_registration: Res<ChannelRegistration>,
    mut commands: Commands,
) {
    commands.insert_resource(create_renet_server(&channel_registration));
    commands.insert_resource(create_transport());
}

fn create_renet_server(channel_registration: &ChannelRegistration) -> RenetServer {
    // Создаем конфиг
    let config = ConnectionConfig {
        available_bytes_per_tick: 60_000,
        client_channels_config: channel_registration.client_channels.clone(),
        server_channels_config: channel_registration.server_channels.clone(),
    };
    RenetServer::new(config)
}
fn create_transport() -> NetcodeServerTransport {
    let server_addr = format!("{LOCALHOST}:{DEFAULT_PORT}").parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };
    NetcodeServerTransport::new(server_config, socket).unwrap()
}

/// Обрабатывает события подключения / отключения клиентов.
fn handle_events(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("Client {client_id} connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}