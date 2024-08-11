use bevy::prelude::States;

/// Состояние первичной загрузки.
#[derive(States, Hash, PartialEq, Eq, Default, Clone, Debug)]
pub enum LoadingState {
    #[default]
    Loading,
    Loaded,
}