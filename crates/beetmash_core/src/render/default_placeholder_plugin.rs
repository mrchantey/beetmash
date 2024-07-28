use crate::prelude::*;
use beetmash_scene::prelude::*;
use bevy::prelude::*;
use bevy::text::CosmicBuffer;

#[derive(Clone)]
pub struct DefaultPlaceholderPlugin;

impl Plugin for DefaultPlaceholderPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			bundle_placeholder_plugin,
			AssetPlaceholderPlugin::<Image>::default(),
			ReadyOnAssetLoadPlugin::<Image>::default(),
			AssetPlaceholderPlugin::<Mesh>::default(),
			ReadyOnAssetLoadPlugin::<Mesh>::default(),
			AssetPlaceholderPlugin::<StandardMaterial>::default(),
			ReadyOnAssetLoadPlugin::<StandardMaterial>::default(),
		))
		.register_type::<AssetLoadBlockAppReady>();


		// temp, something like this will probs be in 0.14.1
		app.world_mut().register_component_hooks::<Text>().on_add(
			|mut world, entity, _| {
				world
					.commands()
					.entity(entity)
					.insert(CosmicBuffer::default());
			},
		);
	}
}
