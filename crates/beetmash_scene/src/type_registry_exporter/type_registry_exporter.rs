use crate::prelude::*;
use anyhow::Result;
use bevy::app::Plugins;
use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use std::path::PathBuf;

/// A helper for exporting the type registry.
/// By default this **will clear the target directory**.
pub struct TypeRegistryExporter<P, M> {
	pub plugin: P,
	pub path: PathBuf,
	phantom: std::marker::PhantomData<M>,
}

impl<P: Clone + Plugins<M>, M> TypeRegistryExporter<P, M> {
	pub fn new(plugin: P) -> Self {
		Self {
			plugin,
			path: PathBuf::from("target/registries/type_registry.json"),
			phantom: std::marker::PhantomData,
		}
	}

	pub fn with_dir(mut self, dir: &str) -> Self {
		self.path = PathBuf::from(dir).join(self.path.file_name().unwrap());
		self
	}

	/// Override the default `type_registry.json` file name.
	pub fn with_name(mut self, name: &str) -> Self {
		self.path.set_file_name(name);
		self
	}

	pub fn export(&self) -> Result<()> {
		let mut app = App::new();
		app.init_resource::<AppTypeRegistry>();
		app.add_plugins(self.plugin.clone()).finish();

		let registry = app.world().resource::<AppTypeRegistry>().read();
		let registry: &TypeRegistry = &registry;
		let serde_registry: SerdeTypeRegistry = registry.into();

		let bytes = serde_json::to_string_pretty(&serde_registry)?;
		let num_bytes = bytes.len();
		let num_kilobytes = num_bytes as i64 / 1024;
		if let Some(parent) = self.path.parent() {
			std::fs::create_dir_all(parent).ok();
		}
		std::fs::write(&self.path, bytes)?;

		println!(
			"Exported type registry\nPath: {}\nsize: {}KB",
			self.path.display(),
			num_kilobytes
		);
		Ok(())
	}
}
