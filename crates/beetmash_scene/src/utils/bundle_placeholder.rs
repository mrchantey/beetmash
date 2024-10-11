use bevy::prelude::*;
use bevy::text::LineBreak;


#[derive(Debug, Clone, Reflect)]
pub enum MeshPlaceholder {
	Circle(Circle),
	Cuboid(Cuboid),
	Cylinder(Cylinder),
	Plane3d(Plane3d),
}

impl From<Circle> for MeshPlaceholder {
	fn from(circle: Circle) -> Self { MeshPlaceholder::Circle(circle) }
}
impl From<Cuboid> for MeshPlaceholder {
	fn from(cuboid: Cuboid) -> Self { MeshPlaceholder::Cuboid(cuboid) }
}
impl From<Cylinder> for MeshPlaceholder {
	fn from(cylinder: Cylinder) -> Self { MeshPlaceholder::Cylinder(cylinder) }
}
impl From<Plane3d> for MeshPlaceholder {
	fn from(plane: Plane3d) -> Self { MeshPlaceholder::Plane3d(plane) }
}


impl Into<Mesh> for MeshPlaceholder {
	fn into(self) -> Mesh {
		match self {
			MeshPlaceholder::Circle(circle) => circle.into(),
			MeshPlaceholder::Cuboid(cuboid) => cuboid.into(),
			MeshPlaceholder::Cylinder(cylinder) => cylinder.into(),
			MeshPlaceholder::Plane3d(plane) => plane.into(),
		}
	}
}

#[derive(Debug, Clone, Reflect)]
pub enum MaterialPlaceholder {
	Color(Color),
	StandardMaterial { base_color: Color, unlit: bool },
}

impl Into<StandardMaterial> for MaterialPlaceholder {
	fn into(self) -> StandardMaterial {
		match self {
			MaterialPlaceholder::Color(color) => color.into(),
			MaterialPlaceholder::StandardMaterial { base_color, unlit } => {
				StandardMaterial {
					base_color,
					unlit,
					..Default::default()
				}
			}
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
	Text {
		sections: Vec<TextSection>,
		style: Style,
		visibility: Visibility,
		linebreak: Option<LineBreak>,
		background_color: Option<Color>,
	},
	Sprite(String),
	Scene(String),
	Pbr {
		mesh: MeshPlaceholder,
		material: MaterialPlaceholder,
	},
}

impl BundlePlaceholder {
	pub fn text_from_sections(sections: Vec<TextSection>) -> Self {
		BundlePlaceholder::Text {
			sections,
			style: default(),
			visibility: default(),
			linebreak: default(),
			background_color: default(),
		}
	}
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
				entity_commands.insert((Camera2d::default(), transform));
			}
			BundlePlaceholder::Camera3d => {
				entity_commands.insert((Camera3d::default(), transform));
			}
			BundlePlaceholder::PointLight => {
				entity_commands.insert((
					PointLight {
						shadows_enabled: true,
						..default()
					},
					transform,
				));
			}
			BundlePlaceholder::Text {
				sections,
				style,
				visibility,
				linebreak,
				background_color,
			} => {
				let mut bundle =
					TextBundle::from_sections(sections).with_style(style);
				bundle.visibility = visibility;
				if let Some(linebreak) = linebreak {
					bundle.text.linebreak = linebreak;
				}
				if let Some(backgrond_color) = background_color {
					bundle.background_color = backgrond_color.into();
				}
				bundle.transform = transform;
				entity_commands.insert(bundle);
			}
			BundlePlaceholder::Sprite(path) => {
				entity_commands.insert(SpriteBundle {
					texture: asset_server.load(path),
					transform,
					..default()
				});
			}
			BundlePlaceholder::Scene(path) => {
				entity_commands
					.insert((SceneRoot(asset_server.load(path)), transform));
			}
			BundlePlaceholder::Pbr { mesh, material } => {
				entity_commands.insert((
					Mesh3d(meshes.add(mesh)),
					MeshMaterial3d(materials.add(material)),
					transform,
				));
			}
		}
	}
}
