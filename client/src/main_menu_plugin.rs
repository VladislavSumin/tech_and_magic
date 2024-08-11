use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::Align2;
use feature_game_state::GameState;
use feature_network_client::ConnectEvent;

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
    mut game_state: ResMut<NextState<GameState>>,
) {
    let mut host = String::new();
    host.push_str("localhost");

    egui::Window::new("Main menu.")
        .pivot(Align2::CENTER_CENTER)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Host");
            ui.text_edit_singleline(&mut host);
            if ui.button("Connect").clicked() {
                // TODO не очень хорошо что мы должны тут и создать подключение
                // и изменить игровое состояние. Получается нужно уведомить две системы.
                game_state.set(GameState::ConnectingToServer);
                ev_connect.send(ConnectEvent {
                    host: "127.0.0.1".into(),
                    port: 5000,
                });
            }
        });
}
