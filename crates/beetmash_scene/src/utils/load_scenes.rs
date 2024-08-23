use crate::prelude::*;
use anyhow::Result;
use bevy::prelude::*;

/// Pass scene paths as arguments to load them on startup.
/// ```sh
/// cargo run -- path/to/scene1.json path/to/scene2.json
/// ```
pub struct CliSceneLoadPlugin;

impl Plugin for CliSceneLoadPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, load_scenes_from_args);
	}
}


fn load_scenes_from_args(_world: &mut World) {
	#[cfg(not(target_arch = "wasm32"))]
	{
		let args: Vec<String> = std::env::args().collect();
		load_scenes(_world, args).expect("Error loading scenes from cli args");
	}
}

#[allow(unused)]
fn load_scenes(world: &mut World, args: Vec<String>) -> Result<()> {
	// The first argument is the path to the program
	for path in args.iter().skip(1) {
		log::info!("Loading scene from: {path}");
		let scene = std::fs::read_to_string(path).map_err(|e| {
			anyhow::anyhow!("\nError reading scene file:\n{path}\n{e}\n")
		})?;
		let format = SceneFormat::from_path(path)?;
		let event = SpawnSceneFile::new(format, scene);
		event.spawn(world)?;
	}
	Ok(())
}
