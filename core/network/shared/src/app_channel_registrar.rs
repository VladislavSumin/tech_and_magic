//! Набор расширений для [App] позволяющих удобным образом регистрировать каналы в момент инициализации плагина.
//! TODO добавить пример использования.

use bevy::app::App;
use crate::channel_registration::{ChannelRegistrar, ChannelRegistrationInfo};

/// Позволяет регистрировать каналы в [App] делегирует работу [ChannelRegistrar].
pub trait AppChannelRegistrar {
    fn register_client_channel<T: AsRef<str>>(
        &mut self, channel_name: T,
        channel: ChannelRegistrationInfo,
    ) -> &mut Self;
    fn register_server_channel<T: AsRef<str>>(
        &mut self, channel_name: T,
        channel: ChannelRegistrationInfo,
    ) -> &mut Self;
}

impl AppChannelRegistrar for App {
    fn register_client_channel<T: AsRef<str>>(
        &mut self, channel_name: T,
        channel: ChannelRegistrationInfo,
    ) -> &mut Self {
        self.world_mut().resource_mut::<ChannelRegistrar>().register_client(channel_name, channel);
        self
    }

    fn register_server_channel<T: AsRef<str>>(
        &mut self, channel_name: T,
        channel: ChannelRegistrationInfo,
    ) -> &mut Self {
        self.world_mut().resource_mut::<ChannelRegistrar>().register_server(channel_name, channel);
        self
    }
}
