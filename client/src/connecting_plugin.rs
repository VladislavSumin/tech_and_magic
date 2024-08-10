use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::Align2;
use crate::game_state::GameState;

pub struct ConnectingPlugin;

impl Plugin for ConnectingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, render.run_if(in_state(GameState::ConnectingToServer)))
        ;
    }
}

fn render(
    mut contexts: EguiContexts,
) {
    egui::Window::new("Connecting to server....")
        .pivot(Align2::CENTER_CENTER)
        .show(contexts.ctx_mut(), |_| {});
}
