use anyhow::Result;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use std::path::Path;

// such a hack
const NUM_IGNORED_RESOURCES: usize = 138;

const ALLOWED_IGNORES: &[&str] = &[
	"bevy_text::text::CosmicBuffer",
	"beet_flow::observers::action_observer_map::ActionObserverMap",
	"bevy_ecs::observer::entity_observer::ObservedBy",
];


pub fn get_save_entities<Q: QueryFilter>(world: &mut World) -> Vec<Entity> {
	// TODO removed ,Without<Observer<OnUserMessage,()>>), check thats ok
	world.query_filtered::<Entity, Q>().iter(world).collect()
}

#[derive(Debug, Clone)]
pub struct DynamicSceneChecks {
	pub asset_checks: bool,
	pub entity_checks: bool,
	pub component_checks: bool,
}
impl Default for DynamicSceneChecks {
	fn default() -> Self { Self::new() }
}

impl DynamicSceneChecks {
	pub fn new() -> Self {
		Self {
			asset_checks: true,
			entity_checks: true,
			component_checks: true,
		}
	}

	pub fn with_asset_checks(mut self, checks: bool) -> Self {
		self.asset_checks = checks;
		self
	}
	pub fn with_entity_checks(mut self, checks: bool) -> Self {
		self.entity_checks = checks;
		self
	}

	pub fn assert_scene_match<Q: QueryFilter>(
		&self,
		path: &Path,
		world: &mut World,
		scene: &DynamicScene,
	) -> Result<()> {
		let mut issues = Vec::<String>::new();

		if self.entity_checks {
			issues.extend(self.check_entities::<Q>(world, scene));
		}

		if self.asset_checks {
			issues.extend(self.check_assets(world, scene));
		}

		if self.component_checks {
			issues.extend(self.check_components(world, scene));
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


	fn check_entities<Q: QueryFilter>(
		&self,
		world: &mut World,
		scene: &DynamicScene,
	) -> Vec<String> {
		let mut issues = Vec::new();
		let num_entities_world = get_save_entities::<Q>(world).len();
		let num_entities_scene = scene.entities.len();
		if num_entities_world != num_entities_scene {
			issues.push(
			format!("Entity count mismatch: Expected {num_entities_world}, got {num_entities_scene}"));
		}
		issues
	}
	fn check_components(
		&self,
		world: &mut World,
		scene: &DynamicScene,
	) -> Vec<String> {
		let mut issues = Vec::new();

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
					issues.push(format!(
						"Component missing: {}",
						component.name()
					));
				}
			}
		}
		issues
	}
	fn check_assets(
		&self,
		world: &mut World,
		scene: &DynamicScene,
	) -> Vec<String> {
		let mut issues = Vec::new();
		let num_resources_world =
			world.iter_resources().count() - NUM_IGNORED_RESOURCES;
		let num_resources_scene = scene.resources.len();
		if num_resources_world != num_resources_scene {
			issues.push(
			format!("Resource count mismatch: Expected {num_resources_world}, got {num_resources_scene}\nRemember to update NUM_IGNORED_RESOURCES when registering assets, events etc."));
		}
		for (resource, _) in world.iter_resources() {
			let resource_scene = scene.resources.iter().find(|r| {
				r.get_represented_type_info()
					.expect("found resource without typeinfo")
					.type_id() == resource
					.type_id()
					.expect("found resource without typeid")
			});
			if resource_scene.is_none() {
				issues.push(format!("Resource missing: {}", resource.name()));
			}
		}
		issues
	}
}
