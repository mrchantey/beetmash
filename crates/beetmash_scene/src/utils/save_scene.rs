use crate::prelude::*;
use anyhow::Result;
use bevy::audio::DefaultSpatialScale;
use bevy::ecs::query::QueryFilter;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::pbr::PointLightShadowMap;
use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;
use std::fs;
use std::path::Path;




/// Saves scenes,attempting to deny any unintended resources and entities.
pub fn save_scene<Q: QueryFilter>(
	world: &mut World,
	checks: &DynamicSceneChecks,
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

	checks.assert_scene_match::<Q>(path, world, &scene)?;

	let type_registry = world.resource::<AppTypeRegistry>();
	let serialized_scene = scene.serialize(&type_registry.read())?;

	if let Some(dir_path) = path.parent() {
		fs::create_dir_all(dir_path)?;
	}
	fs::write(path, serialized_scene.as_bytes())?;

	Ok(())
}

