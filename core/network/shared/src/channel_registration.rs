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
    client_channels: BTreeMap<String, ChannelRegistrationInfo>,
    server_channels: BTreeMap<String, ChannelRegistrationInfo>,
}

/// Информация о конкретном канале.
/// Смотреть [ChannelRegistrar].
pub struct ChannelRegistrationInfo;

/// Информация о каналах, доступна только после инициализации приложения. На этапе регистрации каналы следует добавлять
/// через [ChannelRegistrar]
#[derive(Resource)]
pub struct ChannelRegistration {
    client_channels: HashMap<String, u8>,
    server_channels: HashMap<String, u8>,
}

impl Plugin for NetworkChannelRegistrationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ChannelRegistrar>()
        ;
    }
}

impl ChannelRegistrar {
    /// Регистрирует новый канал client -> server.
    /// [channel_name] должен быть уникален.
    pub fn register_client<T: AsRef<str>>(&mut self, channel_name: T, channel: ChannelRegistrationInfo) {
        ChannelRegistrar::register(&mut self.client_channels, channel_name, channel)
    }

    /// Регистрирует новый канал server -> client.
    /// [channel_name] должен быть уникален.
    pub fn register_server<T: AsRef<str>>(&mut self, channel_name: T, channel: ChannelRegistrationInfo) {
        ChannelRegistrar::register(&mut self.server_channels, channel_name, channel)
    }

    fn register<T: AsRef<str>>(channels: &mut BTreeMap<String, ChannelRegistrationInfo>, channel_name: T, channel: ChannelRegistrationInfo) {
        let is_new = channels.insert(channel_name.as_ref().to_owned(), channel).is_none();
        assert!(!is_new, "Channel with name {} already registered", channel_name.as_ref())
    }
}