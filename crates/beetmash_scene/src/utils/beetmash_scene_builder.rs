use crate::prelude::*;
use anyhow::Result;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use std::path::Path;
use std::path::PathBuf;

pub struct BeetmashSceneBuilder<P> {
	plugin: P,
	dir: PathBuf,
	scenes: Vec<BeetmashScene>,
}

impl Default for BeetmashSceneBuilder<MostDefaultPlugins> {
	fn default() -> Self {
		Self {
			plugin: MostDefaultPlugins,
			dir: PathBuf::from("target/scenes"),
			scenes: Vec::new(),
		}
	}
}


impl<P: Clone + Plugin> BeetmashSceneBuilder<P> {
	pub fn new(plugin: P) -> Self {
		Self {
			plugin,
			dir: PathBuf::from("target/scenes"),
			scenes: Vec::new(),
		}
	}

	pub fn with_dir(mut self, dir: impl Into<PathBuf>) -> Self {
		self.dir = dir.into();
		self
	}

	pub fn add_scene(mut self, scene: BeetmashScene) -> Self {
		self.scenes.push(scene);
		self
	}

	pub fn build(self) -> Result<()> {
		self.scenes
			.into_iter()
			.map(|scene| scene.save(self.plugin.clone(), &self.dir))
			.collect::<Result<Vec<_>>>()?;
		Ok(())
	}
}

pub struct BeetmashScene {
	pub name: &'static str,
	pub system: SystemConfigs,
}

impl BeetmashScene {
	pub fn new<M>(
		name: &'static str,
		system: impl IntoSystemConfigs<M>,
	) -> Self {
		Self {
			name,
			system: system.into_configs(),
		}
	}
	pub fn save(self, plugin: impl Plugin, dir: &Path) -> Result<()> {
		let mut app = App::new();
		app.add_plugins(plugin).finish();

		Schedule::default()
			.add_systems(self.system)
			.run(app.world_mut());

		let filename = format!("{}.ron", self.name);
		let path = dir.join(filename);

		save_scene(app.world_mut(), &path)
	}
}