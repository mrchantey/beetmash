use crate::prelude::*;
use anyhow::Result;
use bevy::app::Plugins;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use std::path::PathBuf;


/// A helper for exporting scenes. The default output directory is `scenes`
/// and by default it will be cleared on export.
pub struct SceneGroupExporter<P, M, Q = ()> {
	plugin: P,
	scenes: Vec<SceneExporter>,
	phantom: std::marker::PhantomData<(M, Q)>,
	config: SceneExportConfig,
}

impl<P: Clone + Plugins<M>, M> SceneGroupExporter<P, M, ()> {
	pub fn new(plugin: P) -> Self {
		Self {
			plugin,
			config: Default::default(),
			scenes: Vec::new(),
			phantom: std::marker::PhantomData,
		}
	}
}
impl<P: Clone + Plugins<M>, M, Q: QueryFilter> SceneGroupExporter<P, M, Q> {
	/// Specify the `[QueryFilter]` for the entities that will be exported.
	pub fn with_filter<Q2: QueryFilter>(self) -> SceneGroupExporter<P, M, Q2> {
		SceneGroupExporter {
			plugin: self.plugin,
			config: self.config,
			scenes: self.scenes,
			phantom: std::marker::PhantomData,
		}
	}

	pub fn without_clear_target(mut self) -> Self {
		self.config.clear_target_dir = false;
		self
	}

	pub fn with_dir(mut self, dir: impl Into<PathBuf>) -> Self {
		self.config.dir = dir.into();
		self
	}
	pub fn with_checks(mut self, checks: DynamicSceneChecks) -> Self {
		self.config.checks = checks;
		self
	}

	pub fn with_config(mut self, config: SceneExportConfig) -> Self {
		self.config = config;
		self
	}

	pub fn add_scene<Marker>(
		mut self,
		name: impl Into<String>,
		builder: impl IntoSystemConfigs<Marker>,
	) -> Self {
		self.scenes.push(SceneExporter::new(name.into(), builder));
		self
	}

	pub fn export(self) -> Result<()> {
		if self.config.clear_target_dir {
			std::fs::remove_dir_all(&self.config.dir).ok();
		}

		self.scenes
			.into_iter()
			.map(|scene| {
				scene.export::<Q, _>(self.plugin.clone(), &self.config)
			})
			.collect::<Result<Vec<_>>>()?;
		Ok(())
	}
}
