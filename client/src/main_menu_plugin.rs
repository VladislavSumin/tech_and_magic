use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport};
use egui::Align2;
use feature_network_client::ConnectEvent;
use crate::game_state::GameState;

/// Плагин отвечающий за работу главного игрового меню.
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, render.run_if(in_state(GameState::MainMenu)))
        ;
    }
}


fn render(
    mut contexts: EguiContexts,
    mut ev_connect: EventWriter<ConnectEvent>,
) {
    let mut host = String::new();
    host.push_str("localhost");

    egui::Window::new("Main menu.")
        .pivot(Align2::CENTER_CENTER)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Host");
            ui.text_edit_singleline(&mut host);
            if ui.button("Connect").clicked() {
                ev_connect.send(ConnectEvent {
                    host: "127.0.0.1".into(),
                    port: 5000,
                });
            }
        });
}
