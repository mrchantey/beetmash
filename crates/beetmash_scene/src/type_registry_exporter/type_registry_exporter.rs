use crate::prelude::*;
use anyhow::Result;
use bevy::app::Plugins;
use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use std::path::PathBuf;

/// A helper for exporting the type registry.
/// By default this **will clear the target directory**.
pub struct TypeRegistryExporter<P, M> {
	plugin: P,
	name: String,
	dir: PathBuf,
	phantom: std::marker::PhantomData<M>,
}

impl<P: Clone + Plugins<M>, M> TypeRegistryExporter<P, M> {
	pub fn new(plugin: P) -> Self {
		Self {
			plugin,
			name: "type_registry".to_string(),
			dir: PathBuf::from("target/type_registries"),
			phantom: std::marker::PhantomData,
		}
	}

	pub fn with_name(mut self, name: &str) -> Self {
		self.name = name.to_string();
		self
	}

	pub fn export(&self) -> Result<()> {
		let mut app = App::new();
		app.init_resource::<AppTypeRegistry>();
		app.add_plugins(self.plugin.clone()).finish();

		let registry = app.world().resource::<AppTypeRegistry>().read();
		let registry: &TypeRegistry = &registry;
		let serde_registry: SerdeTypeRegistry = registry.into();

		let registry_path = self.dir.join(&format!("{}.json", self.name));
		let bytes = serde_json::to_string_pretty(&serde_registry)?;
		let num_bytes = bytes.len();
		let num_kilobytes = num_bytes as i64 / 1024;
		std::fs::create_dir_all(&self.dir).ok();
		std::fs::write(&registry_path, bytes)?;

		println!(
			"type registry exported\npath: {}\nsize: {}KB",
			registry_path.display(),
			num_kilobytes
		);
		Ok(())
	}
}
