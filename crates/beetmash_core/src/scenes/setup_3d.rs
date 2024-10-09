use beetmash_scene::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn camera_3d(mut commands: Commands) {
	commands.spawn(BundlePlaceholder::Camera3d);
}

pub fn ground_3d(mut commands: Commands) {
	commands.spawn(BundlePlaceholder::Pbr {
		mesh: Plane3d::new(Vec3::Y, Vec2::splat(50.)).into(),
		material: Color::srgb(0.3, 0.5, 0.3).into(),
	});
}


pub fn lighting_3d(mut commands: Commands) {
	// Light
	commands.spawn((
		DirectionalLight {
			shadows_enabled: true,
			..default()
		},
		Transform::from_rotation(Quat::from_euler(
			EulerRot::ZYX,
			0.0,
			1.0,
			-PI / 4.,
		)),
		CascadeShadowConfigBuilder {
			first_cascade_far_bound: 20.0,
			maximum_distance: 40.0,
			..default()
		}
		.build(),
	));
}
