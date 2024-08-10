use bevy::log::{Level, LogPlugin};
use bevy::utils::default;

/// Создает [LogPlugin] с настройками по умолчанию.
pub fn create_log_plugin() -> LogPlugin {
    LogPlugin {
        filter: make_log_filter(),
        level: Level::TRACE,
        ..default()
    }
}

/// Возвращает строку с фильтром для логов.
fn make_log_filter() -> String {
    [
        "trace",
        "bevy_core=info",
        "bevy_app=info",
        "bevy_render=info",
        "bevy_winit=info",
        "bevy_time=info",
        "naga=info",
        "gilrs=info",
        "wgpu_core=info",
        "wgpu_hal=info",
        "winit=info",
        "renetcode=debug",
    ]
        .join(",")
}
