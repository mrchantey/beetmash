use crate::prelude::*;
use anyhow::Result;
use bevy::ecs::entity::EntityHashMap;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::scene::serde::SceneDeserializer;
use forky::prelude::*;
use serde::de::DeserializeSeed;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

// we use events because observers are not allowed world access
// https://github.com/bevyengine/bevy/issues/14507
pub fn spawn_scene_file_plugin(app: &mut App) {
	app.add_event::<SpawnSceneFile>()
		.add_event::<SpawnSceneFileResponse>()
		.add_systems(Update, handle_spawn_scene);
}

/// Received by this app, containing the raw text of a file for
/// deserialization and spawning
#[derive(Debug, Clone, Serialize, Deserialize, Event, Reflect)]
pub struct SpawnSceneFile {
	pub format: SceneFormat,
	pub payload: String,
}

impl SpawnSceneFile {
	pub fn new(format: SceneFormat, payload: String) -> Self {
		Self { format, payload }
	}
	pub fn ron(payload: String) -> Self { Self::new(SceneFormat::Ron, payload) }
	pub fn json(payload: String) -> Self {
		Self::new(SceneFormat::Json, payload)
	}
	pub fn spawn(&self, world: &mut World) -> Result<EntityHashMap<Entity>> {
		match self.format {
			SceneFormat::Ron => {
				let mut deserializer =
					bevy::scene::ron::de::Deserializer::from_str(
						&self.payload,
					)?;
				return write_to_world(world, &mut deserializer);
			}
			SceneFormat::Json => {
				let mut deserializer =
					serde_json::Deserializer::from_str(&self.payload);
				return write_to_world(world, &mut deserializer);
			}
		}
	}
}




/// Sent by this app, containing the entity hash map for the spawned scene
#[derive(Debug, Clone, Serialize, Deserialize, Event, Reflect)]
pub struct SpawnSceneFileResponse(pub EntityHashMap<Entity>);

// TODO use observers when we get exclusive observer systems
pub fn handle_spawn_scene(
	world: &mut World,
	events: &mut SystemState<(
		EventReader<SpawnSceneFile>,
		EventWriter<SpawnSceneFileResponse>,
	)>,
) {
	events
		.get_mut(world)
		.0
		.read()
		.map(|e| e.clone())
		.collect::<Vec<_>>()
		.into_iter()
		.map(|scene| scene.spawn(world))
		.collect::<Result<Vec<_>>>()
		.ok_or(|e| log::error!("{e}"))
		.map(|entity_maps| {
			let (_, mut responses) = events.get_mut(world);
			for map in entity_maps {
				responses.send(SpawnSceneFileResponse(map));
			}
		});
}

#[cfg(test)]
mod test {
	use crate::prelude::*;
	use anyhow::Result;
	use bevy::log::LogPlugin;
	use bevy::prelude::*;
	use sweet::*;

	#[derive(Debug, Component, Reflect, PartialEq)]
	#[reflect(Component)]
	struct MyStruct(pub u32);

	#[test]
	fn works() -> Result<()> {
		let mut app = App::new();
		app.register_type::<MyStruct>();

		app.world_mut().spawn(MyStruct(7));
		let scene = DynamicScene::from_world(app.world());
		let str = scene
			.serialize(&app.world().resource::<AppTypeRegistry>().read())?;

		let mut app2 = App::new();

		// DefaultReplicatePlugin removed, why was it here?

		app2.add_plugins((LogPlugin::default(), spawn_scene_file_plugin))
			.add_systems(Update, handle_spawn_scene)
			.add_event::<SpawnSceneFileResponse>()
			.register_type::<MyStruct>();

		app2.world_mut().send_event(SpawnSceneFile::ron(str));

		expect(
			app2.world_mut()
				.query::<&MyStruct>()
				.iter(app2.world())
				.count(),
		)
		.to_be(0)?;
		app2.update();
		expect(
			app2.world_mut()
				.query::<&MyStruct>()
				.iter(app2.world())
				.next(),
		)
		.to_be(Some(&MyStruct(7)))?;

		Ok(())
	}
}

fn write_to_world<'de, D: Deserializer<'de>>(
	world: &mut World,
	deserializer: D,
) -> Result<EntityHashMap<Entity>> {
	let type_registry = world.resource::<AppTypeRegistry>().clone();
	let scene_deserializer = SceneDeserializer {
		type_registry: &type_registry.read(),
	};
	let scene = scene_deserializer
		.deserialize(deserializer)
		.map_err(|e| anyhow::anyhow!("{}", e))?;
	let mut entity_map = Default::default();
	scene.write_to_world(world, &mut entity_map)?;
	Ok(entity_map)
}
