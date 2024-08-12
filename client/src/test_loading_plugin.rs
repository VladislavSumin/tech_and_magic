use bevy::prelude::*;
use core_loading::{LoadingState, LoadingStatus, LoadingStatuses};

/// Создает тестовую загрузку.
pub struct TestLoadingPlugin;

impl Plugin for TestLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, register)
            .add_systems(Update, update.run_if(in_state(LoadingState::Loading)))
        ;
    }
}

const TEST_LOADING_STATUS: &str = "test_loading_status";

fn register(
    mut loading_statuses: ResMut<LoadingStatuses>,
) {
    let ls = LoadingStatus {
        weight: 1.,
        ..default()
    };
    loading_statuses.register(TEST_LOADING_STATUS, ls);
}

fn update(
    mut loading_statuses: ResMut<LoadingStatuses>,
) {
    let status = loading_statuses.get_status_mut(TEST_LOADING_STATUS);
    status.progress += 0.01;
    if status.progress >= 1.0 { status.is_loaded = true }
}