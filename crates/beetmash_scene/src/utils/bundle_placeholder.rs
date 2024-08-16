use bevy::prelude::*;


#[derive(Debug, Clone, Reflect)]
pub enum MeshPlaceholder {
	Plane3d {
		plane: Plane3d,
		width: f32,
		height: f32,
	},
	Circle {
		radius: f32,
	},
	Cuboid {
		width: f32,
		height: f32,
		depth: f32,
	},
}

impl From<Circle> for MeshPlaceholder {
	fn from(circle: Circle) -> Self {
		MeshPlaceholder::Circle {
			radius: circle.radius,
		}
	}
}
impl From<Cuboid> for MeshPlaceholder {
	fn from(cuboid: Cuboid) -> Self {
		MeshPlaceholder::Cuboid {
			width: cuboid.half_size.x * 2.,
			height: cuboid.half_size.y * 2.,
			depth: cuboid.half_size.z * 2.,
		}
	}
}

impl Into<Mesh> for MeshPlaceholder {
	fn into(self) -> Mesh {
		match self {
			MeshPlaceholder::Plane3d {
				plane,
				width,
				height,
			} => plane.mesh().size(width, height).into(),
			MeshPlaceholder::Circle { radius } => Circle::new(radius).into(),
			MeshPlaceholder::Cuboid {
				width,
				height,
				depth,
			} => Cuboid::new(width, height, depth).into(),
		}
	}
}

#[derive(Debug, Clone, Reflect)]
pub enum MaterialPlaceholder {
	Color(Color),
}

impl Into<StandardMaterial> for MaterialPlaceholder {
	fn into(self) -> StandardMaterial {
		match self {
			MaterialPlaceholder::Color(color) => color.into(),
		}
	}
}

impl Into<MaterialPlaceholder> for Color {
	fn into(self) -> MaterialPlaceholder { MaterialPlaceholder::Color(self) }
}

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub enum BundlePlaceholder {
	Camera2d,
	Camera3d,
	PointLight,
	Sprite(String),
	Scene(String),
	Pbr {
		mesh: MeshPlaceholder,
		material: MaterialPlaceholder,
	},
}

pub fn bundle_placeholder_plugin(app: &mut App) {
	app.add_systems(PreUpdate, init_bundle)
		.register_type::<BundlePlaceholder>();
}

fn init_bundle(
	asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut commands: Commands,
	query: Query<
		(Entity, Option<&Transform>, &BundlePlaceholder),
		Added<BundlePlaceholder>,
	>,
) {
	for (entity, transform, placeholder) in query.iter() {
		let mut entity_commands = commands.entity(entity);
		entity_commands.remove::<BundlePlaceholder>();
		let transform = transform.cloned().unwrap_or_default();

		match placeholder.clone() {
			BundlePlaceholder::Camera2d => {
				entity_commands.insert(Camera2dBundle {
					transform,
					..default()
				});
			}
			BundlePlaceholder::Camera3d => {
				entity_commands.insert(Camera3dBundle {
					transform,
					..default()
				});
			}
			BundlePlaceholder::PointLight => {
				entity_commands.insert(PointLightBundle {
					point_light: PointLight {
						shadows_enabled: true,
						..default()
					},
					transform,
					..default()
				});
			}
			BundlePlaceholder::Sprite(path) => {
				entity_commands.insert(SpriteBundle {
					texture: asset_server.load(path),
					transform,
					..default()
				});
			}
			BundlePlaceholder::Scene(path) => {
				entity_commands.insert(SceneBundle {
					scene: asset_server.load(path),
					transform,
					..default()
				});
			}
			BundlePlaceholder::Pbr { mesh, material } => {
				entity_commands.insert(PbrBundle {
					mesh: meshes.add(mesh),
					material: materials.add(material),
					transform,
					..default()
				});
			}
		}
	}
}
