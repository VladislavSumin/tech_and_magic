//! Данный файл отвечает за возможность регистрации каналов для передачи информации.
//! Используется одновременно клиентом и сервером и позволяет синхронизировать id каналов, создаваемых разными 
//! плагинами.

use std::collections::{BTreeMap};
use bevy::prelude::*;
use bevy::utils::HashMap;

/// Регистрирует [ChannelRegistrar].
pub struct NetworkChannelRegistrationPlugin;

/// Общий ресурс через который происходит регистрация каналов, доступен только на этапе инициализации приложения.
/// Для получения доступа к id каналов после инициализации следует воспользоваться ресурсом [ChannelRegistration].
#[derive(Resource, Default)]
pub struct ChannelRegistrar {
    channels: BTreeMap<String, ChannelRegistrationInfo>,
}

/// Информация о конкретном канале.
/// Смотреть [ChannelRegistrar].
pub struct ChannelRegistrationInfo;

/// Информация о каналах, доступна только после инициализации приложения. На этапе регистрации каналы следует добавлять
/// через [ChannelRegistrar]
#[derive(Resource)]
pub struct ChannelRegistration {
    channels: HashMap<String, u8>,
}

impl Plugin for NetworkChannelRegistrationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ChannelRegistrar>()
        ;
    }
}

impl ChannelRegistrar {
    /// Регистрирует новый канал.
    /// [channel_name] должен быть уникален.
    fn register<T: AsRef<str>>(&mut self, channel_name: T, channel: ChannelRegistrationInfo) {
        let is_new = self.channels.insert(channel_name.as_ref().to_owned(), channel).is_none();
        assert!(!is_new, "Channel with name {} already registered", channel_name.as_ref())
    }
}