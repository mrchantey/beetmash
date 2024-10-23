use crate::prelude::*;
use bevy::prelude::*;



pub fn beetmash_full_plugins(app: &mut App) {
	app.add_plugins((
		BeetmashDefaultPlugins::with_beetmash_assets(),
		DefaultPlaceholderPlugin,
		DefaultReplicatePlugin,
		UiTerminalPlugin,
		temp_patches,
	));
}
