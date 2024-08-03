use crate::prelude::*;
use anyhow::Result;
use bevy::app::Plugins;
use bevy::prelude::*;
use bevy_reflect::TypeRegistry;
use std::path::PathBuf;

/// A helper for exporting the type registry.
/// By default this **will clear the target directory**.
pub struct TypeRegistryExporter<P, M> {
	plugin: P,
	dir: PathBuf,
	export_typescript_bingings: bool,
	phantom: std::marker::PhantomData<M>,
}

impl<P: Clone + Plugins<M>, M> TypeRegistryExporter<P, M> {
	pub fn new(plugin: P) -> Self {
		Self {
			plugin,
			export_typescript_bingings: true,
			dir: PathBuf::from("target/type_registry"),
			phantom: std::marker::PhantomData,
		}
	}

	pub fn export(&self) -> Result<()> {
		let mut app = App::new();
		app.init_resource::<AppTypeRegistry>();
		app.add_plugins(self.plugin.clone()).finish();

		let registry = app.world().resource::<AppTypeRegistry>().read();
		let registry: &TypeRegistry = &registry;
		let serde_registry: SerdeTypeRegistry = registry.into();

		let registry_path = self.dir.join("type_registry.json");
		let bytes = serde_json::to_string_pretty(&serde_registry)?;
		let num_bytes = bytes.len();
		let num_kilobytes = num_bytes as i64 / 1024;
		std::fs::remove_dir_all(&self.dir).ok();
		std::fs::create_dir_all(&self.dir).ok();
		std::fs::write(&registry_path, bytes)?;


		println!(
			"type registry exported\npath: {}\nsize: {}KB",
			registry_path.display(),
			num_kilobytes
		);


		if self.export_typescript_bingings {
			export_ts::<SerdeTypeRegistry>(&self.dir)?;
		}

		Ok(())
	}
}
