use anyhow::Result;
use beetmash::core;
use beetmash::prelude::*;

fn main() -> Result<()> {
	SceneGroupExporter::new((
		MostDefaultPlugins,
		DefaultPlaceholderPlugin,
		UiTerminalPlugin,
		DefaultReplicatePlugin,
		temp_patches,
	))
	.add_scene("app", || {})
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
	.export_with_registries()?;

	Ok(())
}
