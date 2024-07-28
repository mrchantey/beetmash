use crate::prelude::*;
use beetmash_net::prelude::*;
use bevy::prelude::*;

/// Includes default transports for native and wasm targets, as well as
pub struct DefaultReplicatePlugin;


impl Plugin for DefaultReplicatePlugin {
	fn build(&self, app: &mut App) {
		let _ = app;

		app.add_plugins((ReplicatePlugin, CommonEventsPlugin));

		#[cfg(feature = "scene")]
		// SpawnSceneFile
		app.add_event::<SpawnSceneFile>()
			.replicate_event_incoming::<SpawnSceneFile>()
			.add_event::<SpawnSceneFileResponse>()
			.replicate_event_outgoing::<SpawnSceneFileResponse>()
			.add_systems(Update, handle_spawn_scene);

		// .add_event::<SomeCustomEvent>()
		// .replicate_event_incoming::<SomeCustomEvent>()

		#[cfg(feature="render")]
			app.add_event::<SaveScreenshot>()
			.replicate_event_incoming::<SaveScreenshot>()
			.add_systems(Update,screenshot_on_event.run_if(run_if_rendering))
			.add_systems(Update,screenshot_on_keypress.run_if(run_if_rendering))
			// .observe(screenshot_on_event)
			// .observe(screenshot_on_keypress)
				/*-*/;

	}
}
