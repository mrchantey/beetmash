use crate::prelude::*;
use anyhow::Result;
use bevy::app::Plugins;
use bevy::audio::DefaultSpatialScale;
use bevy::ecs::query::QueryFilter;
use bevy::ecs::schedule::SystemConfigs;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::pbr::PointLightShadowMap;
use bevy::prelude::*;
use bevy::scene::serde::SceneSerializer;
use bevy::time::TimeUpdateStrategy;
use std::fs;

pub struct SceneExporter {
	pub name: String,
	pub system: SystemConfigs,
}

impl Into<Vec<SceneExporter>> for SceneExporter {
	fn into(self) -> Vec<SceneExporter> { vec![self] }
}

impl SceneExporter {
	pub fn new<M>(name: String, system: impl IntoSystemConfigs<M>) -> Self {
		Self {
			name,
			system: system.into_configs(),
		}
	}
	pub fn export<Q: QueryFilter, M>(
		self,
		plugins: impl Plugins<M>,
		config: &SceneExportConfig,
	) -> Result<()> {
		let mut app = App::new();
		app.add_plugins(plugins).finish();

		// run the builder system once
		// app.world_mut().run_system_once(self.system);
		Schedule::default()
			.add_systems(self.system)
			.run(app.world_mut());


		let world = app.world_mut();

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

		let filename = format!("{}.{}", self.name, config.format.extension());
		let path = config.dir.join(filename);
		config
			.checks
			.assert_scene_match::<Q>(&path, world, &scene)?;

		let type_registry = world.resource::<AppTypeRegistry>();
		let type_registry = type_registry.read();
		let serializer = SceneSerializer::new(&scene, &type_registry);

		let scene_str = config.format.to_string(&serializer)?;

		if let Some(dir_path) = path.parent() {
			fs::create_dir_all(dir_path)?;
		}
		fs::write(path, scene_str)?;

		Ok(())
	}
}