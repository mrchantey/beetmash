//! This is published as an app to beetmash.com
use beetmash::prelude::*;
use beetmash_core::scenes;
use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins(beetmash_full_plugins)
		.add_systems(Startup, (scenes::camera_2d, scenes::ui_terminal_input))
		.run();
}
