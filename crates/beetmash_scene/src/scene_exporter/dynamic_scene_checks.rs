use anyhow::Result;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use std::path::Path;


pub fn get_save_entities<Q: QueryFilter>(world: &mut World) -> Vec<Entity> {
	// TODO removed ,Without<Observer<OnUserMessage,()>>), check thats ok
	world.query_filtered::<Entity, Q>().iter(world).collect()
}

#[derive(Debug, Clone)]
pub struct DynamicSceneChecks {
	/// Check resource count and types
	pub resource_checks: bool,
	/// Check entity count
	pub entity_checks: bool,
	/// Check component count and types
	pub component_checks: bool,
	/// Allow n number of resources to not be present
	/// in the exported scenes [DefaultPlugins]
	pub num_ignored_resources: usize,
	/// List of components that are allowed to be missing
	pub allowed_ignores: Vec<String>,
}
impl Default for DynamicSceneChecks {
	fn default() -> Self {
		Self {
			resource_checks: true,
			entity_checks: true,
			component_checks: true,
			// hacky way to ignore resources pulled in by [DefaultPlugins]
			num_ignored_resources: 138,
			allowed_ignores: vec![
				"bevy_text::text::CosmicBuffer".to_string(),
				"beet_flow::observers::action_observer_map::ActionObserverMap"
					.to_string(),
				"bevy_ecs::observer::entity_observer::ObservedBy".to_string(),
			],
		}
	}
}

impl DynamicSceneChecks {
	pub fn new() -> Self { Self::default() }

	pub fn with_asset_checks(mut self, checks: bool) -> Self {
		self.resource_checks = checks;
		self
	}
	pub fn with_entity_checks(mut self, checks: bool) -> Self {
		self.entity_checks = checks;
		self
	}

	pub fn with_component_checks(mut self, checks: bool) -> Self {
		self.component_checks = checks;
		self
	}

	pub fn with_allowed_ignores(mut self, ignores: Vec<String>) -> Self {
		self.allowed_ignores = ignores;
		self
	}

	pub fn with_num_ignored_resources(mut self, num: usize) -> Self {
		self.num_ignored_resources = num;
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

		if self.resource_checks {
			issues.extend(self.check_resources(world, scene));
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
					&& !self
						.allowed_ignores
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
	fn check_resources(
		&self,
		world: &mut World,
		scene: &DynamicScene,
	) -> Vec<String> {
		let mut issues = Vec::new();
		let Some(num_resources_world) = world
			.iter_resources()
			.count()
			.checked_sub(self.num_ignored_resources)
		else {
			return vec!["DynamicSceneChecks::num_ignored_resources exeeds those found in the World".to_string()];
		};
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
		issues
	}
}
