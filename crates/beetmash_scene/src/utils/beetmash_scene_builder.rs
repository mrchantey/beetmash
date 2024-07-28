use crate::prelude::*;
use anyhow::Result;
use bevy::app::Plugins;
use bevy::ecs::query::QueryFilter;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use std::path::Path;
use std::path::PathBuf;

pub struct BeetmashSceneBuilder<P, M, Q = ()> {
	plugin: P,
	dir: PathBuf,
	scenes: Vec<BeetmashScene>,
	phantom: std::marker::PhantomData<(M, Q)>,
}

impl<P: Clone + Plugins<M>, M> BeetmashSceneBuilder<P, M, ()> {
	pub fn new(plugin: P) -> Self {
		Self {
			plugin,
			dir: PathBuf::from("target/scenes"),
			scenes: Vec::new(),
			phantom: std::marker::PhantomData,
		}
	}
}
impl<P: Clone + Plugins<M>, M, Q: QueryFilter> BeetmashSceneBuilder<P, M, Q> {
	pub fn with_query<Q2: QueryFilter>(self) -> BeetmashSceneBuilder<P, M, Q2> {
		BeetmashSceneBuilder {
			plugin: self.plugin,
			dir: self.dir,
			scenes: self.scenes,
			phantom: std::marker::PhantomData,
		}
	}

	pub fn with_dir(mut self, dir: impl Into<PathBuf>) -> Self {
		self.dir = dir.into();
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
		self.scenes
			.into_iter()
			.map(|scene| scene.save::<Q, _>(self.plugin.clone(), &self.dir))
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

		save_scene::<Q>(app.world_mut(), &path)
	}
}
