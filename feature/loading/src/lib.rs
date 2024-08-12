//! # Core функционал для возможности инициализации приложения.
//! Создает состояние загрузки [LoadingState].
//! Отслеживает статус загрузки с помощью ресурса [LoadingStatuses], после завершения загрузки переводит состояние в
//! [LoadingState::Loaded] и удаляет ресурс [LoadingStatuses].
//!
//! ## Добавление статусов загрузки.
//! Для добавления статуса загрузки (нужно для удержания приложения в состоянии [LoadingState::Loading] и отображения
//! прогресса загрузки необходимо во время работы [Startup] системы зарегистрировать такие статусы используя
//! [LoadingStatuses]. Как только все [LoadingStatus] будут в загружены приложение перейдет в состояние
//! [LoadingState::Loaded].
//! ```rust
//! use bevy::prelude::*;
//! use feature_loading::*;
//!
//! fn main() {
//!     App::new()
//!         .add_systems(Startup, add_loading_status_system)
//!         .add_systems(
//!             Update,
//!             update_loading_status_system.run_if(in_state(LoadingState::Loading)),
//!         )
//!         .run();
//! }
//!
//! // Добавление
//! fn add_loading_status_system(
//!     mut loading_statuses: ResMut<LoadingStatuses>,
//! ) {
//!     loading_statuses.register("unique_id", LoadingStatus::default());
//! }
//!
//! // Обновление
//! fn update_loading_status_system(
//!     mut loading_statuses: ResMut<LoadingStatuses>,
//! ) {
//!     let ls = loading_statuses.get_status_mut("unique_id");
//!     ls.is_loaded = true;
//! }
//!
//! ```
//! ## Наблюдение за прогрессом загрузки.
//! Что бы узнать текущий статус загрузки существует ресурс [LoadingProgress]. **Внимание** данный ресурс доступен
//! только в состоянии [LoadingState::Loading].
//! ```rust
//! use bevy::prelude::*;
//! use feature_loading::LoadingProgress;
//!
//! fn watch_loading_progress(
//!     progress: Res<LoadingProgress>,
//! ) {
//!     println!("Loading progress: {progress}");
//! }
//! ```

use bevy::prelude::*;
use bevy::utils::HashMap;

/// Отвечает за загрузку приложения.
pub struct LoadingPlugin;

/// Состояние первичной загрузки.
#[derive(States, Hash, PartialEq, Eq, Default, Clone, Debug)]
pub enum LoadingState {
    #[default]
    Loading,
    Loaded,
}

/// Текущий прогресс загрузки в диапазоне от 0 до 1.
#[derive(Resource, Deref, DerefMut, Default)]
pub struct LoadingProgress(f64);

/// Список загружаемых в текущий момент ресурсов.
#[derive(Resource, Default)]
pub struct LoadingStatuses {
    statuses: HashMap<String, LoadingStatus>,
}

#[derive(Default)]
pub struct LoadingStatus {
    pub is_loaded: bool,
    /// Прогресс загрузки в диапазоне от 0 до 1.
    pub progress: f64,
    /// Условный вес, то насколько это система долго грузится относительно других.
    /// Влияет на общий вес [progress] относительно других статусов.
    /// Параметр очень субъективный, но подбирать желательно так что бы примерно соответствовало:
    /// 1 единица веса = 1 секунда.
    pub weight: f64,
}

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<LoadingState>()
            .init_resource::<LoadingStatuses>()
            .init_resource::<LoadingProgress>()
            .add_systems(Update, check_loading_statuses.run_if(in_state(LoadingState::Loading)))
        ;
    }
}

impl LoadingStatuses {
    /// Регистрирует новый статус.
    /// [status_name] должен быть уникален.
    pub fn register<T: AsRef<str>>(&mut self, status_name: T, status: LoadingStatus) {
        let is_new = self.statuses.insert(status_name.as_ref().to_owned(), status).is_none();
        assert!(is_new, "Loading status with name {} already registered", status_name.as_ref());
    }

    pub fn get_status_mut<T: AsRef<str>>(&mut self, status_name: T) -> &mut LoadingStatus {
        self.statuses.get_mut(status_name.as_ref())
            .expect("Loading status not exists")
    }
}

fn check_loading_statuses(
    loading_statuses: Res<LoadingStatuses>,
    mut loading_progress: ResMut<LoadingProgress>,
    mut loading_state: ResMut<NextState<LoadingState>>,
    mut commands: Commands,
) {
    // Прогресс загрузки с учетом веса.
    let mut scaled_progress = 0.;
    let mut total_weight = 0.;
    let mut is_loaded = true;

    for (_, loading_status) in &loading_statuses.statuses {
        scaled_progress += loading_status.progress.clamp(0., 1.) * loading_status.weight;
        total_weight += loading_status.weight;
        if !loading_status.is_loaded { is_loaded = false }
    }

    **loading_progress = if total_weight == 0. {
        is_loaded.into()
    } else {
        scaled_progress / total_weight
    };

    if is_loaded {
        loading_state.set(LoadingState::Loaded);
        // TODO возможно тут могут быть проблемы с асинхронностью, тогда удаление ресурсов нужно перенести.
        commands.remove_resource::<LoadingStatuses>();
        commands.remove_resource::<LoadingProgress>();
    }
}
