use beetmash_scene::utils::CliSceneLoadPlugin;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowPlugin;


const DEFAULT_ASSET_PATH: &str = "assets";

/// Opinionated [DefaultPlugins] to work well with scene-based workflows
/// and uploading to [beetmash.com](https://beetmash.com)
pub struct BeetmashDefaultPlugins {
	#[allow(unused)]
	pub native_asset_path: String,
	#[allow(unused)]
	pub wasm_asset_path: String,
	pub assert_local_assets: bool,
}

impl Default for BeetmashDefaultPlugins {
	fn default() -> Self {
		Self {
			native_asset_path: DEFAULT_ASSET_PATH.into(),
			wasm_asset_path: DEFAULT_ASSET_PATH.into(),
			assert_local_assets: false,
		}
	}
}




impl Plugin for BeetmashDefaultPlugins {
	fn build(&self, app: &mut App) {
		self.assert_local_assets();
		app.add_plugins((
			DefaultPlugins
				.set(beetmash_window_plugin())
				.set(AssetPlugin {
					file_path: self.asset_path(),
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
}


impl BeetmashDefaultPlugins {
	pub fn with_wasm_asset_path(wasm_asset_path: impl Into<String>) -> Self {
		Self {
			wasm_asset_path: wasm_asset_path.into(),
			..default()
		}
	}

	pub fn with_native_asset_path(path: impl Into<String>) -> Self {
		Self {
			native_asset_path: path.into(),
			..default()
		}
	}

	pub fn asset_path(&self) -> String {
		#[cfg(target_arch = "wasm32")]
		// return "/wasm/assets".into();
		// return "https://demo.beetmash.com/wasm/assets".into();
		return self.wasm_asset_path.clone();
		#[cfg(not(target_arch = "wasm32"))]
		return self.native_asset_path.clone();
	}

	pub fn with_beetmash_assets() -> Self {
		Self {
			wasm_asset_path:
				"https://beetmash-public.s3.us-west-2.amazonaws.com/assets"
					.into(),
			// "https://storage.googleapis.com/beet-examples/assets".into(),
			assert_local_assets: true,
			..default()
		}
	}

	pub fn assert_local_assets(&self) {
		#[cfg(target_arch = "wasm32")]
		return;
		#[allow(unreachable_code)]
		if self.assert_local_assets
			&& !std::path::Path::new("assets/README.md").exists()
		{
			panic!(
				r#"
🥁🥁🥁
		
Welcome! Beetmash examples use large assets that are stored remotely. 

Unix three liner:

curl -o ./assets.tar.gz https://beetmash-public.s3.us-west-2.amazonaws.com/assets.tar.gz
tar -xzvf ./assets.tar.gz
rm ./assets.tar.gz

Windows / manual:

1. Download https://beetmash-public.s3.us-west-2.amazonaws.com/assets.tar.gz
2. Unzip into `./assets`

🥁🥁🥁
"#
			);
		}
	}
}

fn close_on_esc(
	mut commands: Commands,
	focused_windows: Query<(Entity, &Window)>,
	input: Res<ButtonInput<KeyCode>>,
) {
	for (window, focus) in focused_windows.iter() {
		if !focus.focused {
			continue;
		}

		if input.just_pressed(KeyCode::Escape) {
			commands.entity(window).despawn();
		}
	}
}



/// Ensure your app looks beautiful on beetmash.com
pub fn beetmash_window_plugin() -> WindowPlugin {
	WindowPlugin {
		primary_window: Some(Window {
			fit_canvas_to_parent: true,
			canvas: Some("#beetmash-canvas".to_string()),
			resizable: true,
			..default()
		}),
		..default()
	}
}
