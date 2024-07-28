//! This is published as an app to beetmash.com
use beetmash::prelude::beetmash_default_plugins;
use beetmash::prelude::BundlePlaceholderPlugin;
use beetmash::prelude::DefaultReplicatePlugin;
use beetmash::prelude::UiTerminalPlugin;
use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins((
			beetmash_default_plugins,
			BundlePlaceholderPlugin,
			DefaultReplicatePlugin,
			UiTerminalPlugin,
		))
		.run();
}

// fn main() {
// 	App::new()
// 		.add_plugins((
// 			beetmash_default_plugins,
// 			BundlePlaceholderPlugin,
// 			DefaultReplicatePlugin,
// 			UiTerminalPlugin,
// 		))
// 		.add_systems(
// 			Startup,
// 			(
// 				beetmash::core::scenes::camera_2d,
// 				beetmash::core::scenes::ui_terminal_input,
// 			),
// 		)
// 		.run();
// }
