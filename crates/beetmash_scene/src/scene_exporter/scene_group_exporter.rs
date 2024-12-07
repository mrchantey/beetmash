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


	/// Akin to a dry run, export scenes to string instead of files.
	pub fn export_to_string(self) -> Result<Vec<String>> {
		self.assert_empty()?;

		self.scenes
			.into_iter()
			.map(|scene| {
				scene
					.export_to_string::<Q, _>(self.plugin.clone(), &self.config)
			})
			.collect::<Result<Vec<_>>>()
	}

	pub fn export(self) -> Result<()> {
		self.assert_empty()?;

		if self.config.clear_target_dir {
			std::fs::remove_dir_all(&self.config.dir).ok();
		}

		let num_scenes = self.scenes.len();

		self.scenes
			.into_iter()
			.map(|scene| {
				scene.export_to_file::<Q, _>(self.plugin.clone(), &self.config)
			})
			.collect::<Result<Vec<_>>>()?;

		println!(
			"Exported scenes\nPath: {}\tscene count: {}KB",
			self.config.dir.display(),
			num_scenes
		);

		Ok(())
	}

	/// A check performed to ensure that no resources or entities are exported
	/// that weren't built by the scene system.
	fn assert_empty(&self) -> Result<()> {
		let empty_scene = SceneExporter::new("empty", || {})
			.export_to_string::<Q, _>(self.plugin.clone(), &self.config)
			.map_err(|err| {
				anyhow::anyhow!("Could not create empty scene: {:?}", err)
			})?;

		let empty = serde_json::from_str::<serde_json::Value>(&empty_scene)?;
		let expected = serde_json::json!({
			"resources": {},
			"entities": {}
		});

		if empty != expected {
			anyhow::bail!("Could not create empty scene, this is usually due to new resources that need to be ignores:\n {:?}", empty);
		}

		Ok(())
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use anyhow::Result;
	use bevy::prelude::*;

	#[test]
	fn succeeds_exporting_empty_scene() -> Result<()> {
		SceneGroupExporter::new(|app: &mut App| {
			app.add_plugins(MostDefaultPlugins);
		})
		.with_config(SceneExportConfig {
			dir: "test".into(),
			checks: DynamicSceneChecks {
				resource_checks: false,
				entity_checks: true,
				component_checks: true,
				..default()
			},
			..default()
		})
		.add_scene("app", || {})
		.export_to_string()?;
		Ok(())
	}
}
