use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_renet::renet::transport::NetcodeTransportError;
use egui::Align2;
use crate::game_state::GameState;

/// Плагин:
/// * отображает ошибку ошибки соединения.
/// * следит за статусом соединения и при ошибке переводит игру в состояние [GameState::ConnectionError].
pub struct ConnectionErrorPlugin;

impl Plugin for ConnectionErrorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    render.run_if(in_state(GameState::ConnectionError)),
                    handle_connection_state,
                ),
            )
        ;
    }
}

fn handle_connection_state(
    mut transport_error_events: EventReader<NetcodeTransportError>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for _ in transport_error_events.read().take(1) {
        game_state.set(GameState::ConnectionError)
    }
}

fn render(
    mut contexts: EguiContexts,
    mut game_state: ResMut<NextState<GameState>>,
) {
    egui::Window::new("Connection error!")
        .pivot(Align2::CENTER_CENTER)
        .show(contexts.ctx_mut(), |ui| {
            if ui.button("Back to main menu").clicked() {
                game_state.set(GameState::MainMenu)
            }
        });
}
