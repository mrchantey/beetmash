use beetmash_scene::utils::CliSceneLoadPlugin;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowPlugin;
use forky_bevy::systems::close_on_esc;

/// A couple of opinionated modifications and additions to [DefaultPlugins]
/// So they are better suited for publishing on beetmash.com
pub fn beetmash_default_plugins(app: &mut App) {
	app.add_plugins((
		DefaultPlugins
			.set(beetmash_window_plugin())
			.set(AssetPlugin {
				// file_path: assets_path(),
				meta_check: AssetMetaCheck::Never,
				..default()
			})
			.build(),
		CliSceneLoadPlugin,
		// WorldInspectorPlugin::default()
		// .run_if(input_toggle_active(false, KeyCode::Tab)),
	))
	.add_systems(Update, close_on_esc);
}



/// Ensure your app looks beautiful on beetmash.com
pub fn beetmash_window_plugin() -> WindowPlugin {
	WindowPlugin {
		primary_window: Some(Window {
			fit_canvas_to_parent: true,
			canvas: Some("beetmash-canvas".to_string()),
			resizable: true,
			..default()
		}),
		..default()
	}
}
