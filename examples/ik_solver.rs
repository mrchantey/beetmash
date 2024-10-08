use beetmash::prelude::*;
use bevy::prelude::*;
use std::f32::consts::TAU;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, setup)
		.add_systems(Update, update)
		.run();
}


#[derive(Resource)]
struct Entities {
	pos_a: Entity,
	pos_b: Entity,
	pos_c: Entity,
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// light
	commands.spawn((
		PointLight {
			shadows_enabled: true,
			..default()
		},
		Transform::from_xyz(4.0, 8.0, 4.0),
	));
	// camera
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
	));

	// cube
	let pos_a = commands
		.spawn((
			Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
			MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 255))),
			Transform::from_xyz(0.0, 0., 0.0),
		))
		.id();
	let pos_b = commands
		.spawn((
			Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
			MeshMaterial3d(materials.add(Color::srgb_u8(255, 255, 0))),
			Transform::from_xyz(0.0, 0.5, 0.0),
		))
		.id();
	let pos_c = commands
		.spawn((
			Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
			MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 255))),
			Transform::from_xyz(0.5, 0.5, 0.0),
		))
		.id();

	commands.insert_resource(Entities {
		pos_a,
		pos_b,
		pos_c,
	});
}

fn update(
	time: Res<Time>,
	entities: Res<Entities>,
	mut transforms: Query<&mut Transform>,
) {
	let theta = time.elapsed_seconds() * TAU;
	let x = theta.cos();
	let y = theta.sin();

	let (mat1, mat2, mat3) = solve_ik_3d(IkOptions3d {
		target: Vec3::new(x, 0., y),
		// target: Vec3::new(0., 100., 0.1),
		len_a: 1.,
		len_b: 1.,
		// extent_min: 0.,
		// extent_max: 1.,
		..default()
	})
	.unwrap();

	let (_, rot1, pos1) = mat1.to_scale_rotation_translation();
	let (_, rot2, pos2) = mat2.to_scale_rotation_translation();
	let (_, rot3, pos3) = mat3.to_scale_rotation_translation();

	let mut tran1 = transforms.get_mut(entities.pos_a).unwrap();
	tran1.translation = pos1;
	tran1.rotation = rot1;
	let mut tran2 = transforms.get_mut(entities.pos_b).unwrap();
	tran2.translation = pos2;
	tran2.rotation = rot2;
	let mut tran3 = transforms.get_mut(entities.pos_c).unwrap();
	tran3.translation = pos3;
	tran3.rotation = rot3;
}
