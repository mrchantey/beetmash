use crate::prelude::*;
use beetmash_net::prelude::*;
use beetmash_scene::prelude::*;
use bevy::prelude::*;

/// Includes default transports for native and wasm targets,
/// as well as common replication events.
#[derive(Debug, Clone)]
pub struct DefaultReplicatePlugin;


impl Plugin for DefaultReplicatePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			ReplicatePlugin,
			CommonEventsPlugin,
			DefaultTransportPlugin,
		));

		#[cfg(feature = "scene")]
		app.add_plugins(spawn_scene_file_plugin)
			.replicate_event_incoming::<SpawnSceneFile>()
			.replicate_event_outgoing::<SpawnSceneFileResponse>();
		// SpawnSceneFile

		// .add_event::<SomeCustomEvent>()
		// .replicate_event_incoming::<SomeCustomEvent>()

		#[cfg(feature="render")]
			app
			.replicate_observer_incoming::<SaveScreenshot>()
			.observe(screenshot_on_event)
			.add_systems(Update, screenshot_on_keypress)
			// .observe(screenshot_on_event)
			// .observe(screenshot_on_keypress)
				/*-*/;
	}
}
