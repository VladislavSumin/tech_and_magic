use bevy::prelude::*;
use core_loading::{LoadingState, LoadingStatus, LoadingStatuses};
use core_loading::app_loading_registrar::AppLoadingRegistrar;

/// Создает тестовую загрузку.
pub struct TestLoadingPlugin;
const TEST_LOADING_STATUS: &str = "test_loading_status";

impl Plugin for TestLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_loading_status(
                TEST_LOADING_STATUS,
                LoadingStatus {
                    weight: 1.,
                    ..default()
                },
            )
            .add_systems(Update, update.run_if(in_state(LoadingState::Loading)))
        ;
    }
}

fn update(
    mut loading_statuses: ResMut<LoadingStatuses>,
) {
    let status = loading_statuses.get_status_mut(TEST_LOADING_STATUS);
    status.progress += 0.01;
    if status.progress >= 1.0 { status.is_loaded = true }
}