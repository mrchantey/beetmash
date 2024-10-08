use bevy::ecs::observer::ObserverState;
use bevy::prelude::*;

/// A common filter to use when exporting scenes.
pub type DefaultSceneExportFilter = (Without<ObserverState>, Without<Observer>);


/// Various fixes for unstable bevy stuff.
pub fn temp_patches(app: &mut App) {
	app.register_type::<ScrollPosition>(); // temp https://github.com/bevyengine/bevy/pull/15721
}
