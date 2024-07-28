use anyhow::Result;
use beetmash::core;
use beetmash::prelude::*;
use bevy::ecs::observer::ObserverState;
use bevy::prelude::*;
use std::fs;

fn main() -> Result<()> {
	BeetmashSceneBuilder::new((
		MostDefaultPlugins,
		BundlePlaceholderPlugin,
		UiTerminalPlugin,
	))
	.with_query::<(Without<ObserverState>, Without<Observer<OnLogMessage, ()>>)>(
	)
	.with_dir("crates/beetmash_core/scenes")
	.add_scene("empty", || {})
	.add_scene("camera-2d", core::scenes::camera_2d)
	.add_scene("camera-3d", core::scenes::camera_3d)
	.add_scene("ui-terminal", core::scenes::ui_terminal)
	.add_scene("ui-terminal-input", core::scenes::ui_terminal_input)
	.build()?;

	// check empty is empty
	let empty = fs::read_to_string("crates/beetmash_core/scenes/empty.ron")?;
	assert_eq!(
		empty,
		"(
  resources: {},
  entities: {},
)"
	);

	Ok(())
}
