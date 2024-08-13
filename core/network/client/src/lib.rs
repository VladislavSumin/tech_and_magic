use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::prelude::*;
use bevy_renet::renet::{ConnectionConfig, RenetClient};
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError};
use bevy_renet::RenetClientPlugin;
use bevy_renet::transport::NetcodeClientPlugin;
use core_loading::LoadingState;
use core_network_shared::channel_registration::{ChannelRegistration, NetworkChannelRegistrationPlugin};

pub use core_network_shared::channel_registration::ChannelMapping;

/// Отвечает за базовую работу сети на клиенте.
/// Плагин слушает события [ConnectEvent] и устанавливает соединение при получении такого события.
pub struct ClientNetworkPlugin;

/// Событие-запрос на установление соединения с сервером.
#[derive(Event)]
pub struct ConnectEvent {
    pub host: String,
    pub port: u16,
}

impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                RenetClientPlugin,
                NetcodeClientPlugin,
                NetworkChannelRegistrationPlugin,
            ))
            .add_event::<ConnectEvent>()
            // TODO подумать, нужно ли тут смотреть на конкретные состояния.
            .add_systems(
                Update,
                (
                    handle_connect_event.run_if(not(resource_exists::<RenetClient>)),
                    handle_transport_error.run_if(resource_exists::<RenetClient>),
                ).run_if(in_state(LoadingState::Loaded)),
            )
        ;
    }
}

/// Слушает события [ConnectEvent] и добавляет необходимые ресурсы при их получении.
fn handle_connect_event(
    channel_registration: Res<ChannelRegistration>,
    mut connect_events: EventReader<ConnectEvent>,
    mut commands: Commands,
) {
    for event in connect_events.read() {
        info!("Connecting to {}:{}", event.host, event.port);

        // Создаем конфиг
        let config = ConnectionConfig {
            available_bytes_per_tick: 60_000,
            client_channels_config: channel_registration.client_channels.clone(),
            server_channels_config: channel_registration.server_channels.clone(),
        };

        // Создаем сетевой клиент.
        let client = RenetClient::new(config);
        commands.insert_resource(client);

        // Создаем сетевой слой для клиента.
        let server_addr = format!("{}:{}", event.host, event.port).parse().unwrap();
        let authentication = ClientAuthentication::Unsecure {
            server_addr,
            client_id: 0,
            user_data: None,
            protocol_id: 0,
        };
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

        commands.insert_resource(transport);
    }
}

/// Слушает события [NetcodeTransportError] и в случае ошибки удаляет сетевой слой.
fn handle_transport_error(
    mut transport_error_events: EventReader<NetcodeTransportError>,
    mut commands: Commands,
) {
    // Специально читаем только одно событие за тик, так как renet кидает их сразу два.
    for error in transport_error_events.read().take(1) {
        warn!("Transport error: {}", error);
        // TODO тут при удалении получаем логи от библиотеки renetcode с ошибками, но как их
        // исправить я пока не знаю, объединение в одну команду не помогает.
        commands.remove_resource::<NetcodeClientTransport>();
        commands.remove_resource::<RenetClient>();
    }
}
