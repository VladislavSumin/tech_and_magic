//! Данный файл отвечает за возможность регистрации каналов для передачи информации.
//! Используется одновременно клиентом и сервером и позволяет синхронизировать id каналов, создаваемых разными 
//! плагинами.

use std::collections::{BTreeMap};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_renet::renet::{ChannelConfig, SendType};
use core_loading::LoadingState;

/// Регистрирует [ChannelRegistrar].
/// Для своей работы требует [LoadingPlugin].
pub struct NetworkChannelRegistrationPlugin;

/// Общий ресурс через который происходит регистрация каналов, доступен только на этапе инициализации приложения.
/// Для получения доступа к id каналов после инициализации следует воспользоваться ресурсом [ChannelMapping].
#[derive(Resource, Default)]
pub struct ChannelRegistrar {
    client_channels: BTreeMap<String, ChannelRegistrationInfo>,
    server_channels: BTreeMap<String, ChannelRegistrationInfo>,
}

/// Информация о конкретном канале.
/// Смотреть [ChannelRegistrar].
pub struct ChannelRegistrationInfo {
    pub send_type: SendType,
    pub max_memory_usage_bytes: usize,
}

/// Информация о каналах, доступна только после инициализации приложения. На этапе регистрации каналы следует добавлять
/// через [ChannelRegistrar].
#[derive(Resource)]
pub struct ChannelMapping {
    client_channels: HashMap<String, u8>,
    server_channels: HashMap<String, u8>,
}

/// Внутренне представление каналов, используется при инициализации сетевого слоя.
#[derive(Resource)]
pub(crate) struct ChannelRegistration {
    client_channels: Vec<ChannelConfig>,
    server_channels: Vec<ChannelConfig>,
}

impl Plugin for NetworkChannelRegistrationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ChannelRegistrar>()
            .add_systems(OnExit(LoadingState::Loading), create_channel_registration)
        ;
    }
}

/// Удаляет ресурс [ChannelRegistrar].
/// Взамен создает два новых [ChannelMapping] и [ChannelRegistration].
fn create_channel_registration(
    channel_registrar: Res<ChannelRegistrar>,
    mut commands: Commands,
) {
    let mut client_channels = vec![];
    let mut client_channel_mapping = HashMap::new();
    for (index, (name, info)) in
        channel_registrar.client_channels.iter().enumerate() {
        client_channels.push(ChannelConfig {
            channel_id: index as u8,
            send_type: info.send_type.clone(),
            max_memory_usage_bytes: info.max_memory_usage_bytes,
        });
        client_channel_mapping.insert(name.clone(), index as u8);
    }

    let mut server_channels = vec![];
    let mut server_channel_mapping = HashMap::new();
    for (index, (name, info)) in
        channel_registrar.server_channels.iter().enumerate() {
        server_channels.push(ChannelConfig {
            channel_id: index as u8,
            send_type: info.send_type.clone(),
            max_memory_usage_bytes: info.max_memory_usage_bytes,
        });
        server_channel_mapping.insert(name.clone(), index as u8);
    }

    commands.insert_resource(ChannelMapping {
        client_channels: client_channel_mapping,
        server_channels: server_channel_mapping,
    });

    commands.insert_resource(ChannelRegistration {
        client_channels,
        server_channels,
    });

    commands.remove_resource::<ChannelRegistrar>()
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
        assert!(is_new, "Channel with name {} already registered", channel_name.as_ref())
    }
}