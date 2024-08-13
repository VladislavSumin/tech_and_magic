//! Набор расширений для [App] позволяющих удобным образом регистрировать каналы в момент инициализации плагина.

use bevy::app::App;
use crate::channel_registration::{ChannelRegistrar, ChannelRegistrationInfo};

pub trait AppChannelRegistrar {
    fn register_client_channel<T: AsRef<str>>(&mut self, channel_name: T, channel: ChannelRegistrationInfo);
    fn register_server_channel<T: AsRef<str>>(&mut self, channel_name: T, channel: ChannelRegistrationInfo);
}

impl AppChannelRegistrar for App {
    fn register_client_channel<T: AsRef<str>>(&mut self, channel_name: T, channel: ChannelRegistrationInfo) {
        self.world_mut().resource_mut::<ChannelRegistrar>().register_client(channel_name, channel);
    }

    fn register_server_channel<T: AsRef<str>>(&mut self, channel_name: T, channel: ChannelRegistrationInfo) {
        self.world_mut().resource_mut::<ChannelRegistrar>().register_server(channel_name, channel);
    }
}
