use beetmash_scene::utils::CliSceneLoadPlugin;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowPlugin;
use forky_bevy::systems::close_on_esc;


const DEFAULT_ASSETS_PATH: &str = "assets";

pub struct BeetmashDefaultPlugins {
	#[allow(unused)]
	wasm_asset_path: String,
	assert_local_assets: bool,
}

impl Default for BeetmashDefaultPlugins {
	fn default() -> Self {
		Self {
			wasm_asset_path: DEFAULT_ASSETS_PATH.into(),
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
					file_path: self.assets_path(),
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
	pub fn new(wasm_asset_path: String) -> Self {
		Self {
			wasm_asset_path,
			assert_local_assets: false,
		}
	}

	pub fn assets_path(&self) -> String {
		#[cfg(target_arch = "wasm32")]
		// return "/wasm/assets".into();
		// return "https://demo.beetmash.com/wasm/assets".into();
		return self.remote_asset_server.clone();
		#[cfg(not(target_arch = "wasm32"))]
		return DEFAULT_ASSETS_PATH.into();
	}

	pub fn with_beetmash_assets() -> Self {
		Self {
			wasm_asset_path:
				"https://storage.googleapis.com/beet-examples/assets".into(),
			assert_local_assets: true,
		}
	}

	pub fn assert_local_assets(&self) {
		#[cfg(target_arch = "wasm32")]
		return;
		if self.assert_local_assets
			&& !std::path::Path::new("assets/README.md").exists()
		{
			panic!(
				r#"
🥁🥁🥁
		
Welcome! Beetmash examples use large assets that are stored remotely. 

Windows:

1. Download https://storage.googleapis.com/beet-misc/assets.tar.gz
2. Unzip into `./assets`

Linux/MacOS:

curl -o ./assets.tar.gz https://storage.googleapis.com/beet-misc/assets.tar.gz
tar -xzvf ./assets.tar.gz
rm ./assets.tar.gz

🥁🥁🥁
"#
			);
		}
	}
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
