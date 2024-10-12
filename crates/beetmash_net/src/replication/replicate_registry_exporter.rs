use crate::prelude::*;
use anyhow::Result;
use bevy::app::Plugins;
use bevy::prelude::*;
use std::fs;
use std::path::PathBuf;

/// Replicated components and resources have unique ids that
/// must be consistent among apps. Use this exporter to share them
pub struct ReplicateRegistryExporter<P, M> {
	pub plugin: P,
	pub path: PathBuf,
	phantom: std::marker::PhantomData<M>,
}

impl<P: Clone + Plugins<M>, M> ReplicateRegistryExporter<P, M> {
	pub fn new(plugin: P) -> Self {
		Self {
			plugin,
			path: PathBuf::from("target/registries/replication_registry.json"),
			phantom: std::marker::PhantomData,
		}
	}

	pub fn with_dir(mut self, dir: &str) -> Self {
		self.path = PathBuf::from(dir).join(self.path.file_name().unwrap());
		self
	}

	/// Override the default `replication_registry.json` file name.
	pub fn with_name(mut self, name: &str) -> Self {
		self.path.set_file_name(name);
		self
	}
	/// Build a replication registry and write it to a file.
	/// Expects the app to have a ReplicateRegistry resource.
	/// # Errors
	/// If failed to write or the resource was not found
	pub fn export(&self) -> Result<()> {
		let mut app = App::new();
		app.add_plugins(self.plugin.clone());
		let world = app.world();

		let registry =
			world.get_resource::<ReplicateRegistry>().ok_or_else(|| {
				anyhow::anyhow!("Failed to get ReplicateRegistry resource")
			})?;
		let json = registry.types_to_json();
		if let Some(parent) = self.path.parent() {
			fs::create_dir_all(parent).ok();
		}
		fs::write(&self.path, json)?;
		println!(
			"Exported replicate registry:\nPath: {}",
			self.path.display(),
			// registry.types_to_json()
		);
		Ok(())
	}
}
