use bevy::prelude::*;
use feature_sync_player_position_shared::SyncPlayerPositionSharedPlugin;

pub struct SyncPlayerPositionPlugin;

impl Plugin for SyncPlayerPositionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SyncPlayerPositionSharedPlugin)
        ;
    }
}