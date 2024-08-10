use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport};
use egui::Align2;
use crate::game_state::GameState;

/// Плагин отвечающий за работу главного игрового меню.
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ConnectEvent>()
            .add_systems(Update, (render, connect).run_if(in_state(GameState::MainMenu)))
        ;
    }
}

#[derive(Event)]
struct ConnectEvent;

fn render(
    mut contexts: EguiContexts,
    mut ev_connect: EventWriter<ConnectEvent>,
) {
    let mut host = String::new();
    host.push_str("localhost");

    egui::Window::new("Main menu.")
        .pivot(Align2::LEFT_TOP)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Host");
            ui.text_edit_singleline(&mut host);
            if ui.button("Connect").clicked() {
                ev_connect.send(ConnectEvent);
            }
        });
}

fn connect(
    mut commands: Commands,
    mut ev_connect: EventReader<ConnectEvent>,
) {
    for ev in ev_connect.read() {
        let authentication = ClientAuthentication::Unsecure {
            server_addr: "127.0.0.1:5000".parse().unwrap(),
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
