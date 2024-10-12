use bevy::prelude::*;



/// Various fixes for unstable bevy stuff.
pub fn temp_patches(app: &mut App) {
	app.register_type::<ScrollPosition>(); // temp https://github.com/bevyengine/bevy/pull/15721
}
