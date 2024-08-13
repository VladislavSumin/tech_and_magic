//! Набор расширений для [App] позволяющих более просто регистрировать [LoadingStatus]:
//! ```rust
//! use bevy::prelude::*; 
//! use core_loading::LoadingStatus; 
//! use core_loading::app_loading_registrar::AppLoadingRegistrar;
//! 
//! fn main() {
//!     App::new()
//!         .register_loading_status("unique_name", LoadingStatus::default())
//!         .run()
//! }
//! ```
use bevy::app::App;
use crate::{LoadingStatus, LoadingStatuses};

pub trait AppLoadingRegistrar {
    fn register_loading_status<T: AsRef<str>>(&mut self, status_name: T, status: LoadingStatus) -> &mut Self;
}

impl AppLoadingRegistrar for App {
    fn register_loading_status<T: AsRef<str>>(&mut self, status_name: T, status: LoadingStatus) -> &mut Self {
        self.world_mut().resource_mut::<LoadingStatuses>().register(status_name, status);
        self
    }
}