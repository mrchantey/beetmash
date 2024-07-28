use anyhow::Result;
use bevy::audio::DefaultSpatialScale;
use bevy::ecs::query::QueryFilter;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::pbr::PointLightShadowMap;
use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;
use std::fs;
use std::path::Path;

// such a hack
const NUM_IGNORED_RESOURCES: usize = 138;


fn get_save_entities<Q: QueryFilter>(world: &mut World) -> Vec<Entity> {
	// TODO removed ,Without<Observer<OnUserMessage,()>>), check thats ok
	world.query_filtered::<Entity, Q>().iter(world).collect()
}

/// Saves scenes,attempting to deny any unintended resources and entities.
pub fn save_scene<Q: QueryFilter>(
	world: &mut World,
	path: &Path,
) -> Result<()> {
	let entities = get_save_entities::<Q>(world);
	// let scene = DynamicScene::from_world(world);
	let scene = DynamicSceneBuilder::from_world(world)
		// render plugin
		.deny_resource::<Msaa>()
		.deny_resource::<ClearColor>()
		.deny_resource::<AmbientLight>()
		.deny_resource::<DirectionalLightShadowMap>()
		.deny_resource::<PointLightShadowMap>()
		.deny_resource::<GlobalVolume>()
		.deny_resource::<DefaultSpatialScale>()
		.deny_resource::<GizmoConfigStore>()
		// time plugin
		.deny_resource::<Time>()
		.deny_resource::<Time<Real>>()
		.deny_resource::<Time<Virtual>>()
		.deny_resource::<Time<Fixed>>()
		.deny_resource::<TimeUpdateStrategy>()
		.extract_entities(entities.into_iter())
		.extract_resources()
		.build();

	assert_scene_match::<Q>(path, world, &scene)?;

	let type_registry = world.resource::<AppTypeRegistry>();
	let serialized_scene = scene.serialize(&type_registry.read())?;

	if let Some(dir_path) = path.parent() {
		fs::create_dir_all(dir_path)?;
	}
	fs::write(path, serialized_scene.as_bytes())?;

	Ok(())
}


const ALLOWED_IGNORES: &[&str] = &[
	"bevy_text::text::CosmicBuffer",
	"beet_flow::observers::action_observer_map::ActionObserverMap",
	"bevy_ecs::observer::entity_observer::ObservedBy",
];

fn assert_scene_match<Q: QueryFilter>(
	path: &Path,
	world: &mut World,
	scene: &DynamicScene,
) -> Result<()> {
	let mut issues = Vec::<String>::new();

	let num_entities_world = get_save_entities::<Q>(world).len();
	let num_entities_scene = scene.entities.len();
	if num_entities_world != num_entities_scene {
		issues.push(
		format!("Entity count mismatch: Expected {num_entities_world}, got {num_entities_scene}"));
	}

	let num_resources_world =
		world.iter_resources().count() - NUM_IGNORED_RESOURCES;
	let num_resources_scene = scene.resources.len();
	if num_resources_world != num_resources_scene {
		issues.push(
		format!("Resource count mismatch: Expected {num_resources_world}, got {num_resources_scene}\nRemember to update NUM_IGNORED_RESOURCES when registering assets, events etc."));
	}
	// for (resource, _) in world.iter_resources() {
	// 	let resource_scene = scene.resources.iter().find(|r| {
	// 		r.get_represented_type_info()
	// 			.expect("found resource without typeinfo")
	// 			.type_id() == resource
	// 			.type_id()
	// 			.expect("found resource without typeid")
	// 	});
	// 	if resource_scene.is_none() {
	// 		issues.push(format!("Resource missing: {}", resource.name()));
	// 	}
	// }

	for dyn_entity in scene.entities.iter() {
		// let scene_entity =
		// 	.expect("just checked entity count");

		for component in world.inspect_entity(dyn_entity.entity) {
			let num_components_world =
				world.inspect_entity(dyn_entity.entity).count();
			let num_components_scene = dyn_entity.components.len();
			if num_components_world != num_components_scene {
				// issues.push(format!(
				// 	"Component count mismatch: Expected {num_components_world}, got {num_components_scene}"
				// ));
				// println!(
				// 	"{filename}: Component count mismatch: Expected {num_components_world}, got {num_components_scene}"
				// );
			}

			let component_scene = dyn_entity.components.iter().find(|c| {
				c.get_represented_type_info()
					.expect("found component without typeinfo")
					.type_id() == component
					.type_id()
					.expect("found component without typeid")
			});
			if component_scene.is_none()
				&& !ALLOWED_IGNORES
					.iter()
					.any(|i| component.name().starts_with(i))
			{
				issues.push(format!("Component missing: {}", component.name()));
			}
		}
	}
	if issues.len() > 0 {
		anyhow::bail!(
			"{}: issues found:\n{}",
			path.display(),
			issues.join("\n")
		)
	} else {
		Ok(())
	}
}
