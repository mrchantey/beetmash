use anyhow::Result;
use beetmash_net::prelude::*;
use beetmash_scene::prelude::*;
use bevy::app::Plugins;
use bevy::ecs::query::QueryFilter;

/// Export all required elements of your app,
/// 1. [SceneGroupExporter]
/// 2. [TypeRegistryExporter]
/// 3. [ReplicateRegistryExporter]
pub struct BeetmashExporter<Plugin, PluginMarker, QueryFilter> {
	pub scene_group_exporter:
		SceneGroupExporter<Plugin, PluginMarker, QueryFilter>,
	pub type_registry: TypeRegistryExporter<Plugin, PluginMarker>,
	pub replicate_registry: ReplicateRegistryExporter<Plugin, PluginMarker>,
}

impl<P: Clone + Plugins<M>, M, Q: QueryFilter> BeetmashExporter<P, M, Q> {
	pub fn new(plugin: P) -> Self {
		Self {
			scene_group_exporter: SceneGroupExporter::new(plugin.clone())
				.with_filter::<Q>(),
			type_registry: TypeRegistryExporter::new(plugin.clone()),
			replicate_registry: ReplicateRegistryExporter::new(plugin),
		}
	}
	pub fn export(self) -> Result<()> {
		self.scene_group_exporter.export()?;
		self.type_registry.export()?;
		self.replicate_registry.export()?;
		Ok(())
	}
}
