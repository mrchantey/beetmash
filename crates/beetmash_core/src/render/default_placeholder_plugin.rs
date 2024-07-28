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
			// sprites
			AssetPlaceholderPlugin::<Image>::default(),
			ReadyOnAssetLoadPlugin::<Image>::default(),
			// 3d
			AssetPlaceholderPlugin::<Mesh>::default(),
			ReadyOnAssetLoadPlugin::<Mesh>::default(),
			AssetPlaceholderPlugin::<StandardMaterial>::default(),
			ReadyOnAssetLoadPlugin::<StandardMaterial>::default(),
			// animation
			AnimationGraphPlaceholderPlugin,
			AssetPlaceholderPlugin::<AnimationClip>::default(),
			ReadyOnAssetLoadPlugin::<AnimationClip>::default(),
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
