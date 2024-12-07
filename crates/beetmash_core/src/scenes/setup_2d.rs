use crate::prelude::*;
use beetmash_scene::prelude::*;
use bevy::prelude::*;

pub fn camera_2d(mut commands: Commands) {
	commands.spawn(BundlePlaceholder::Camera2d);
}


pub fn space_scene(mut commands: Commands) {
	commands.spawn((
		AssetLoadBlockAppReady,
		Transform::from_translation(Vec3::new(0., 0., -1.))
			.with_scale(Vec3::splat(100.)),
		BundlePlaceholder::Sprite {
			path: "space_background/Space_Stars2.png".into(),
			image_mode: SpriteImageMode::Tiled {
				tile_x: true,
				tile_y: true,
				stretch_value: 0.01,
			},
		},
	));
}
