use anyhow::Result;
use beetmash::core;
use beetmash::prelude::*;
use bevy::ecs::observer::ObserverState;
use bevy::prelude::*;
use std::fs;

const DIR: &str = "scenes";

fn main() -> Result<()> {
	SceneExporter::new((
		MostDefaultPlugins,
		DefaultPlaceholderPlugin,
		UiTerminalPlugin,
	))
	.with_query::<(Without<ObserverState>, Without<Observer<OnLogMessage, ()>>)>(
	)
	.with_dir(DIR)
	.add_scene("empty", || {})
	// ui
	.add_scene("ui-terminal", core::scenes::ui_terminal)
	.add_scene("ui-terminal-input", core::scenes::ui_terminal_input)
	// 2d
	.add_scene("camera-2d", core::scenes::camera_2d)
	.add_scene("space-scene", core::scenes::space_scene)
	// 3d
	.add_scene("camera-3d", core::scenes::camera_3d)
	.add_scene("lighting-3d", core::scenes::lighting_3d)
	.add_scene("ground-3d", core::scenes::ground_3d)
	.export()?;

	// check empty is empty
	assert_eq!(
		fs::read_to_string("scenes/empty.ron")?,
		"(
  resources: {},
  entities: {},
)"
	);

	Ok(())
}
