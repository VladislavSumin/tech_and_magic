//! # Client players
//! Создает и обновляет список подключенных игроков [Players]. Для каждого игрока создает [Entity] и управляет ее
//! жизненным циклом, на эти сущности можно вешать дополнительные компоненты.
//! **Внимание** текущий игрок тоже будет представлен в этом списке.
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_renet::renet::ClientId;
use core_players_shared::SharedPlayersPlugin;

pub struct ClientPlayersPlugin;

impl Plugin for ClientPlayersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SharedPlayersPlugin)
            .init_resource::<Players>()
        ;
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Players(HashMap<ClientId, Entity>);
