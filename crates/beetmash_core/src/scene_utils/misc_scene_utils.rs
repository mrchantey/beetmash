use beetmash_scene::prelude::*;
use bevy::app::Plugins;
use bevy::ecs::observer::ObserverState;
use bevy::ecs::system::SystemIdMarker;
use bevy::prelude::*;

/// A common filter to use when exporting scenes.
pub type DefaultSceneExportFilter = (
	Without<ObserverState>,
	Without<Observer>,
	Without<SystemIdMarker>,
);


/// Various fixes for unstable bevy stuff.
pub fn temp_patches(app: &mut App) {
	app.register_type::<ScrollPosition>(); // temp https://github.com/bevyengine/bevy/pull/15721
}

#[extend::ext(name=SceneGroupExporterExt)]
pub impl<P: Clone + Plugins<M>, M>
	SceneGroupExporter<P, M, DefaultSceneExportFilter>
{
	fn new(plugin: P) -> Self {
		SceneGroupExporter::new_no_filter(plugin)
			.with_filter::<DefaultSceneExportFilter>()
	}
}
