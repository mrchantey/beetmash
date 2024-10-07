use crate::prelude::*;
use beetmash_net::events::AppReady;
use bevy::ecs::observer::ObserverState;
use bevy::prelude::*;

/// A common filter to use when exporting scenes.
pub type DefaultSceneExportFilter = (
	Without<ObserverState>,
	Without<Observer<OnLogMessage, ()>>,
	Without<Observer<AppReady, ()>>,
);
