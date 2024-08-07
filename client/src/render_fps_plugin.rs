use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

/// Плагин для отображения FPS в правом верхнем углу.
/// Для работы требует подключенный плагин [FrameTimeDiagnosticsPlugin].
pub struct RenderFpsPlugin;

impl Plugin for RenderFpsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_fps_text_texture)
            .add_systems(Update, update_fps)
        ;
    }
}

#[derive(Component)]
struct FpsCounter;
fn setup_fps_text_texture(
    mut commands: Commands,
) {
    commands.spawn((
        TextBundle::from_section("FPS", TextStyle::default())
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(8.0),
                left: Val::Px(8.0),
                ..default()
            }),
        FpsCounter,
    ));
}

fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_query: Query<&mut Text, With<FpsCounter>>,
) {
    let mut text = fps_query.single_mut();
    let fps = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
        .unwrap()
        .smoothed()
        .unwrap_or(0f64);
    text.sections[0].value = format!("FPS: {:.0}", fps);
}