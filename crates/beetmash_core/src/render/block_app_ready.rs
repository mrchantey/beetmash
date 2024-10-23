use beetmash_net::prelude::*;
use beetmash_scene::utils::HandleWrapper;
use bevy::prelude::*;
use std::marker::PhantomData;

/// Add this component alongside an asset handle to block the [`AppReady`] event from being triggered.
#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct AssetLoadBlockAppReady;

pub struct ReadyOnAssetLoadPlugin<A: Asset>(PhantomData<A>);

impl<A: Asset> Default for ReadyOnAssetLoadPlugin<A> {
	fn default() -> Self { Self(PhantomData) }
}

impl<A: Asset> Plugin for ReadyOnAssetLoadPlugin<A> {
	fn build(&self, app: &mut App) {
		app.add_systems(PreUpdate, ready_on_asset_load::<A>)
			.register_type::<AssetLoadBlockAppReady>();
	}
}

pub fn ready_on_asset_load<A: Asset>(
	mut asset_events: EventReader<AssetEvent<A>>,
	mut commands: Commands,
	query: Query<(Entity, &HandleWrapper<A>), With<AssetLoadBlockAppReady>>,
	all_blocks: Query<Entity, With<AssetLoadBlockAppReady>>,
) {
	let mut total_ready = 0;
	for ev in asset_events.read() {
		match ev {
			AssetEvent::LoadedWithDependencies { id } => {
				for (entity, handle) in query.iter() {
					if handle.id() == *id {
						commands
							.entity(entity)
							.remove::<AssetLoadBlockAppReady>();
						total_ready += 1;
					}
				}
			}
			_ => {}
		}
	}
	let total_blocks = all_blocks.iter().count();
	if total_blocks > 0 && total_blocks == total_ready {
		commands.trigger(AppReady);
	}
}
