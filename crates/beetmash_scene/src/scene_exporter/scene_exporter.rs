use crate::prelude::*;
use anyhow::Result;
use bevy::app::Plugins;
use bevy::ecs::query::QueryFilter;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use bevy::scene::serde::SceneSerializer;
use std::fs;

pub struct SceneExporter {
	pub name: String,
	pub system: SystemConfigs,
}

impl Into<Vec<SceneExporter>> for SceneExporter {
	fn into(self) -> Vec<SceneExporter> { vec![self] }
}

impl SceneExporter {
	pub fn new<M>(
		name: impl Into<String>,
		system: impl IntoSystemConfigs<M>,
	) -> Self {
		Self {
			name: name.into(),
			system: system.into_configs(),
		}
	}
	pub fn export_to_string<Q: QueryFilter, M>(
		self,
		plugins: impl Plugins<M>,
		config: &SceneExportConfig,
	) -> Result<String> {
		let mut app = App::new();
		app.add_plugins(plugins).finish();

		// run the builder system once
		// app.world_mut().run_system_once(self.system);
		Schedule::default()
			.add_systems(self.system)
			.run(app.world_mut());
		app.world_mut().flush();


		let world = app.world_mut();

		let entities = get_save_entities::<Q>(world);
		// let scene = DynamicScene::from_world(world);
		let scene = config
			.checks
			.filtered_builder(world)
			.extract_entities(entities.into_iter())
			.extract_resources()
			.build();

		config
			.checks
			.assert_scene_match::<Q>(&self.name, world, &scene)?;

		let type_registry = world.resource::<AppTypeRegistry>();
		let type_registry = type_registry.read();
		let serializer = SceneSerializer::new(&scene, &type_registry);

		let scene_str = config.format.to_string(&serializer)?;

		Ok(scene_str)
	}

	pub fn export_to_file<Q: QueryFilter, M>(
		self,
		plugins: impl Plugins<M>,
		config: &SceneExportConfig,
	) -> Result<()> {
		let filename = format!("{}.{}", self.name, config.format.extension());
		
		let scene_str = self.export_to_string::<Q, M>(plugins, config)?;

		let path = config.dir.join(filename);
		if let Some(dir_path) = path.parent() {
			fs::create_dir_all(dir_path)?;
		}
		fs::write(path, scene_str)?;
		Ok(())
	}
}
