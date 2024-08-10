use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::prelude::*;
use bevy_renet::renet::{ConnectionConfig, RenetClient};
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError};
use bevy_renet::RenetClientPlugin;
use bevy_renet::transport::NetcodeClientPlugin;

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
            ))
            .add_event::<ConnectEvent>()
            // TODO подумать, нужно ли тут смотреть на конкретные состояния.
            .add_systems(Update, (handle_connect_event, handle_transport_error))
        ;
    }
}

/// Слушает события [ConnectEvent] и добавляет необходимые ресурсы при их получении.
fn handle_connect_event(
    mut connect_events: EventReader<ConnectEvent>,
    mut commands: Commands,
) {
    for event in connect_events.read() {
        info!("Connecting to {}:{}", event.host, event.port);
        // Создаем сетевой клиент.
        let client = RenetClient::new(ConnectionConfig::default());
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
    for error in transport_error_events.read() {
        warn!("Transport error: {}", error);
        commands.remove_resource::<NetcodeClientTransport>();
        commands.remove_resource::<RenetClient>();
    }
}
