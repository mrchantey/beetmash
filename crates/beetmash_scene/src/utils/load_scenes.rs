use anyhow::Result;
use bevy::ecs::entity::EntityHashMap;
use bevy::prelude::*;
use bevy::scene::serde::SceneDeserializer;
use serde::de::DeserializeSeed;

/// Pass scene paths as arguments to load them on startup.
/// ```sh
/// cargo run -- path/to/scen1.ron path/to/scene2.ron
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
		let scene = std::fs::read_to_string(path)?;
		write_ron_to_world(&scene, world)?;
	}
	Ok(())
}


/// Convenience wrapper for writing ron strings to the world.
pub fn write_ron_to_world(
	ron_str: &str,
	world: &mut World,
) -> Result<EntityHashMap<Entity>> {
	let type_registry = world.resource::<AppTypeRegistry>().clone();
	let mut deserializer =
		bevy::scene::ron::de::Deserializer::from_str(ron_str)?;
	let scene_deserializer = SceneDeserializer {
		type_registry: &type_registry.read(),
	};
	let scene = scene_deserializer
		.deserialize(&mut deserializer)
		.map_err(|e| deserializer.span_error(e))?;
	let mut entity_map = Default::default();
	scene.write_to_world(world, &mut entity_map)?;
	Ok(entity_map)
}
