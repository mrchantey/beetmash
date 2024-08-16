//! a serializable version of the bevy 3d scene example
//! https://bevyengine.org/examples-webgpu/3d-rendering/3d-scene/
use beetmash::prelude::*;
use bevy::prelude::*;

fn main() {
	SceneGroupExporter::new(register_types)
		.add_scene("my_base_scene", spawn_simple_environment)
		.add_scene("my_beautiful_scene", spawn_simple_scene)
		.with_checks(DynamicSceneChecks::new().with_num_ignored_resources(6))
		.export()
		.unwrap();

	ReplicateRegistryExporter::new(DefaultReplicatePlugin)
		.export()
		.unwrap();
}

fn register_types(app: &mut App) {
	app.register_type::<Transform>()
		.register_type::<BundlePlaceholder>();
}

pub fn spawn_simple_environment(mut commands: Commands) {
	// light
	commands.spawn((
		BundlePlaceholder::PointLight,
		Transform::from_xyz(4.0, 8.0, 4.0),
	));
	// camera
	commands.spawn((
		BundlePlaceholder::Camera3d,
		Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
	));
}

pub fn spawn_simple_scene(mut commands: Commands) {
	// circular base
	commands.spawn((
		BundlePlaceholder::Pbr {
			mesh: Circle::new(4.0).into(),
			material: Color::WHITE.into(),
		},
		Transform::from_rotation(Quat::from_rotation_x(
			-std::f32::consts::FRAC_PI_2,
		)),
	));
	// cube
	commands.spawn((
		BundlePlaceholder::Pbr {
			mesh: Cuboid::new(1.0, 1.0, 1.0).into(),
			material: Color::srgb_u8(124, 144, 255).into(),
		},
		Transform::from_xyz(0.0, 0.5, 0.0),
	));
}
