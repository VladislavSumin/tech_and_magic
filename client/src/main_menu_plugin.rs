use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::Align2;
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

fn render(mut contexts: EguiContexts) {
    let mut host = String::new();
    host.push_str("localhost");

    egui::Window::new("Main menu.")
        .pivot(Align2::LEFT_TOP)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Host");
            ui.text_edit_singleline(&mut host);
            if ui.button("Connect").clicked() {
                //TODO
            }
        });
}
