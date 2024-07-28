use crate::prelude::*;
use anyhow::Result;
use bevy::app::Plugins;
use bevy::ecs::query::QueryFilter;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use std::path::Path;
use std::path::PathBuf;

/// A helper for exporting scenes.
/// By default this **will clear the target directory**.
pub struct SceneExporter<P, M, Q = ()> {
	plugin: P,
	dir: PathBuf,
	scenes: Vec<BeetmashScene>,
	phantom: std::marker::PhantomData<(M, Q)>,
	clear_target_dir: bool,
	checks: DynamicSceneChecks,
}

impl<P: Clone + Plugins<M>, M> SceneExporter<P, M, ()> {
	pub fn new(plugin: P) -> Self {
		Self {
			plugin,
			dir: PathBuf::from("target/scenes"),
			scenes: Vec::new(),
			phantom: std::marker::PhantomData,
			clear_target_dir: true,
			checks: default(),
		}
	}
}
impl<P: Clone + Plugins<M>, M, Q: QueryFilter> SceneExporter<P, M, Q> {
	pub fn with_query<Q2: QueryFilter>(self) -> SceneExporter<P, M, Q2> {
		SceneExporter {
			plugin: self.plugin,
			dir: self.dir,
			scenes: self.scenes,
			phantom: std::marker::PhantomData,
			clear_target_dir: self.clear_target_dir,
			checks: self.checks,
		}
	}

	pub fn with_dir(mut self, dir: impl Into<PathBuf>) -> Self {
		self.dir = dir.into();
		self
	}
	pub fn with_checks(mut self, checks: DynamicSceneChecks) -> Self {
		self.checks = checks;
		self
	}

	pub fn add_scene<Marker>(
		mut self,
		name: impl Into<String>,
		builder: impl IntoSystemConfigs<Marker>,
	) -> Self {
		self.scenes.push(BeetmashScene::new(name.into(), builder));
		self
	}

	pub fn build(self) -> Result<()> {
		if self.clear_target_dir {
			std::fs::remove_dir_all(&self.dir).ok();
		}

		self.scenes
			.into_iter()
			.map(|scene| {
				scene.save::<Q, _>(self.plugin.clone(), &self.checks, &self.dir)
			})
			.collect::<Result<Vec<_>>>()?;
		Ok(())
	}
}

pub struct BeetmashScene {
	pub name: String,
	pub system: SystemConfigs,
}

impl Into<Vec<BeetmashScene>> for BeetmashScene {
	fn into(self) -> Vec<BeetmashScene> { vec![self] }
}

impl BeetmashScene {
	pub fn new<M>(name: String, system: impl IntoSystemConfigs<M>) -> Self {
		Self {
			name,
			system: system.into_configs(),
		}
	}
	pub fn save<Q: QueryFilter, M>(
		self,
		plugins: impl Plugins<M>,
		checks: &DynamicSceneChecks,
		dir: &Path,
	) -> Result<()> {
		let mut app = App::new();
		app.add_plugins(plugins).finish();

		// run the builder system once
		// app.world_mut().run_system_once(self.system);
		Schedule::default()
			.add_systems(self.system)
			.run(app.world_mut());

		let filename = format!("{}.ron", self.name);
		let path = dir.join(filename);

		save_scene::<Q>(app.world_mut(), checks, &path)
	}
}
