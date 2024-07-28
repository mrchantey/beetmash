//! This is published as an app to beetmash.com
use beetmash::prelude::*;
use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins((
			BeetmashDefaultPlugins::with_beetmash_assets(),
			DefaultPlaceholderPlugin,
			DefaultReplicatePlugin,
			UiTerminalPlugin,
		))
		.run();
}
