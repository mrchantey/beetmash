use crate::prelude::*;
use anyhow::Result;
use bevy::app::Plugins;
use bevy::ecs::observer::ObserverState;
use bevy::ecs::query::QueryFilter;
use bevy::ecs::system::SystemIdMarker;
use bevy::prelude::*;
use std::path::PathBuf;


/// A helper for exporting scenes. The default output directory is `scenes`
/// and by default it will be cleared on export.
pub struct SceneGroupExporter<P, M, Q = ()> {
	pub plugin: P,
	pub scenes: Vec<SceneExporter>,
	pub phantom: std::marker::PhantomData<(M, Q)>,
	pub config: SceneExportConfig,
}

/// A common filter to use when exporting scenes.
pub type DefaultSceneExportFilter = (
	Without<ObserverState>,
	Without<Observer>,
	Without<SystemIdMarker>,
);

// #[extend::ext(name=SceneGroupExporterExt)]
impl<P: Clone + Plugins<M>, M>
	SceneGroupExporter<P, M, DefaultSceneExportFilter>
{
	pub fn new(plugin: P) -> Self {
		SceneGroupExporter::new_no_filter(plugin)
			.with_filter::<DefaultSceneExportFilter>()
	}
}

impl<P: Clone + Plugins<M>, M> SceneGroupExporter<P, M, ()> {
	pub fn new_no_filter(plugin: P) -> Self {
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
	/// Increment the number of ignored resources by `count`.
	pub fn add_ignored_resources(mut self, count: isize) -> Self {
		self.config.checks.num_ignored_resources = self
			.config
			.checks
			.num_ignored_resources
			.saturating_add_signed(count);
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
		self.assert_empty()?;

		if self.config.clear_target_dir {
			std::fs::remove_dir_all(&self.config.dir).ok();
		}

		self.scenes
			.into_iter()
			.map(|scene| {
				scene.export_to_file::<Q, _>(self.plugin.clone(), &self.config)
			})
			.collect::<Result<Vec<_>>>()?;

		Ok(())
	}

	/// A check performed to ensure that no resources or entities are exported
	/// that weren't built by the scene system.
	fn assert_empty(&self) -> Result<()> {
		let empty_scene = SceneExporter::new("empty", || {})
			.export_to_string::<Q, _>(self.plugin.clone(), &self.config)?;

		let empty = serde_json::from_str::<serde_json::Value>(&empty_scene)?;
		let expected = serde_json::json!({
			"resources": {},
			"entities": {}
		});

		if empty != expected {
			anyhow::bail!("Could not create empty scene:\n {:?}", empty);
		}

		Ok(())
	}
}
