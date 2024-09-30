use beetmash::prelude::*;
use bevy::prelude::*;

/// In scene-based workflows apps are mainly for adding plugins and systems.
/// They should only spawn low-level entities and resources, allowing for
/// scenes to be spawned via some IO mechanism, ie cli, fs or network.
pub fn base_app_plugin(app: &mut App) {
	app.add_plugins((
		BeetmashDefaultPlugins::default(),
		DefaultPlaceholderPlugin,
		DefaultReplicatePlugin,
	));
}
