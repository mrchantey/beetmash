use bevy::prelude::*;

#[derive(Debug, Clone, Reflect)]
pub enum MeshPlaceholder {
	Circle(Circle),
	Cuboid(Cuboid),
	Sphere(Sphere),
	Cylinder(Cylinder),
	Plane3d(Plane3d),
}

impl From<Circle> for MeshPlaceholder {
	fn from(shape: Circle) -> Self { MeshPlaceholder::Circle(shape) }
}
impl From<Cuboid> for MeshPlaceholder {
	fn from(shape: Cuboid) -> Self { MeshPlaceholder::Cuboid(shape) }
}
impl From<Cylinder> for MeshPlaceholder {
	fn from(shape: Cylinder) -> Self { MeshPlaceholder::Cylinder(shape) }
}
impl From<Plane3d> for MeshPlaceholder {
	fn from(shape: Plane3d) -> Self { MeshPlaceholder::Plane3d(shape) }
}
impl From<Sphere> for MeshPlaceholder {
	fn from(shape: Sphere) -> Self { MeshPlaceholder::Sphere(shape) }
}


impl Into<Mesh> for MeshPlaceholder {
	fn into(self) -> Mesh {
		match self {
			MeshPlaceholder::Circle(shape) => shape.into(),
			MeshPlaceholder::Cuboid(shape) => shape.into(),
			MeshPlaceholder::Cylinder(shape) => shape.into(),
			MeshPlaceholder::Sphere(shape) => shape.into(),
			MeshPlaceholder::Plane3d(shape) => shape.into(),
		}
	}
}

#[derive(Debug, Clone, Reflect)]
pub enum MaterialPlaceholder {
	Color(Color),
	StandardMaterial {
		base_color: Color,
		unlit: bool,
	},
	Texture {
		path: String,
		unlit: bool,
		alpha_mode: AlphaMode,
	},
}

impl MaterialPlaceholder {
	pub fn unlit(base_color: impl Into<Color>) -> Self {
		MaterialPlaceholder::StandardMaterial {
			base_color: base_color.into(),
			unlit: true,
		}
	}

	pub fn into_material(self, asset_server: &AssetServer) -> StandardMaterial {
		match self {
			MaterialPlaceholder::Color(color) => color.into(),
			MaterialPlaceholder::StandardMaterial { base_color, unlit } => {
				StandardMaterial {
					base_color,
					unlit,
					..Default::default()
				}
			}
			MaterialPlaceholder::Texture {
				path,
				unlit,
				alpha_mode,
			} => StandardMaterial {
				base_color: Color::WHITE,
				unlit,
				alpha_mode,
				base_color_texture: Some(asset_server.load(path).clone()),
				..Default::default()
			},
		}
	}
}

impl Into<MaterialPlaceholder> for Color {
	fn into(self) -> MaterialPlaceholder { MaterialPlaceholder::Color(self) }
}
impl Into<MaterialPlaceholder> for Srgba {
	fn into(self) -> MaterialPlaceholder {
		MaterialPlaceholder::Color(self.into())
	}
}

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub enum BundlePlaceholder {
	Camera2d,
	Camera3d,
	PointLight,
	Text {
		sections: Vec<String>,
		node: Node,
		visibility: Visibility,
		layout: TextLayout,
		background_color: Option<Color>,
	},
	Sprite {
		path: String,
		image_mode: SpriteImageMode,
	},
	Scene(String),
	Gltf(String),
	Pbr {
		mesh: MeshPlaceholder,
		material: MaterialPlaceholder,
	},
}

impl BundlePlaceholder {
	pub fn text(sections: Vec<String>) -> Self {
		BundlePlaceholder::Text {
			sections,
			node: default(),
			visibility: default(),
			layout: default(),
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
	query: Query<(Entity, &BundlePlaceholder), Added<BundlePlaceholder>>,
) {
	for (entity, placeholder) in query.iter() {
		let mut entity_commands = commands.entity(entity);
		entity_commands.remove::<BundlePlaceholder>();
		match placeholder.clone() {
			BundlePlaceholder::Camera2d => {
				entity_commands.insert(Camera2d::default());
			}
			BundlePlaceholder::Camera3d => {
				entity_commands.insert(Camera3d::default());
			}
			BundlePlaceholder::PointLight => {
				entity_commands.insert(PointLight {
					shadows_enabled: true,
					..default()
				});
			}
			BundlePlaceholder::Text {
				sections,
				node,
				visibility,
				layout,
				background_color,
			} => {
				// create a tree if more than one section
				if sections.len() == 1 {
					entity_commands.insert(Text::new(sections[0].clone()));
				} else {
					entity_commands.insert(TextLayout::default());
					for section in sections {
						entity_commands.with_children(|parent| {
							parent.spawn(TextSpan::new(section.clone()));
						});
					}
				}
				entity_commands.insert((node, visibility, layout));
				if let Some(background_color) = background_color {
					entity_commands.insert(BackgroundColor(background_color));
				}
			}
			BundlePlaceholder::Sprite { path, image_mode } => {
				entity_commands.insert(Sprite {
					image: asset_server.load(path),
					image_mode,
					..default()
				});
			}
			BundlePlaceholder::Scene(path) => {
				entity_commands.insert(SceneRoot(asset_server.load(path)));
			}
			BundlePlaceholder::Gltf(path) => {
				entity_commands.insert(SceneRoot(
					asset_server
						.load(GltfAssetLabel::Scene(0).from_asset(path)),
				));
			}
			BundlePlaceholder::Pbr { mesh, material } => {
				entity_commands.insert((
					Mesh3d(meshes.add(mesh)),
					MeshMaterial3d(
						materials.add(material.into_material(&asset_server)),
					),
				));
			}
		}
	}
}
