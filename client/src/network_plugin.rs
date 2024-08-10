use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;
use bevy_renet::transport::NetcodeClientPlugin;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                RenetClientPlugin,
                NetcodeClientPlugin,
            ))
        ;
    }
}