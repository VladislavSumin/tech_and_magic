use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<LoadingState>()
        ;
    }
}


#[derive(States, Hash, PartialEq, Eq, Default, Clone, Debug)]
pub enum LoadingState {
    #[default]
    Loading,
    Loded,
}