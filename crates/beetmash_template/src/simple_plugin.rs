use crate::prelude::*;
use beetmash::prelude::*;
use bevy::prelude::*;

/// In scene-based workflows apps are mainly for adding plugins and systems.
/// They should only spawn low-level entities and resources, allowing for
/// scenes to be spawned via some IO mechanism, ie fs or network.
pub fn simple_plugin(app: &mut App) {
	app.add_plugins((
		DefaultPlugins.set(my_window_plugin()),
		PlaceholdersPlugin,
		DefaultReplicatePlugin,
		CliSceneLoadPlugin,
	));
}

/// Beetmash creates a canvas for us, the id can be configured in `Cargo.toml`
fn my_window_plugin() -> WindowPlugin {
	WindowPlugin {
		primary_window: Some(Window {
			canvas: Some("#beetmash-canvas".into()),
			..default()
		}),
		..default()
	}
}
