use anyhow::Result;
use beetmash::core;
use beetmash::prelude::*;
use std::fs;

fn main() -> Result<()> {
	SceneGroupExporter::new((
		MostDefaultPlugins,
		DefaultPlaceholderPlugin,
		UiTerminalPlugin,
	))
	.with_filter::<DefaultSceneExportFilter>()
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

	let empty = serde_json::from_str::<serde_json::Value>(
		&fs::read_to_string("scenes/empty.json")?,
	)?;
	let expected = serde_json::json!({
		"resources": {},
		"entities": {}
	});

	assert_eq!(empty, expected);

	Ok(())
}
