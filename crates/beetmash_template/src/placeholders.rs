use bevy::prelude::*;

/// Assets and cameras are not yet serializable, here are some placeholders
pub struct PlaceholdersPlugin;

pub struct RegisterPlaceholderTypes;

impl Plugin for RegisterPlaceholderTypes {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraPlaceholder>()
            .register_type::<PointLightPlaceholder>()
            .register_type::<PbrPlaceholder>();
    }
}

impl Plugin for PlaceholdersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RegisterPlaceholderTypes);

        let world = app.world_mut();

        world
            .register_component_hooks::<CameraPlaceholder>()
            .on_add(|mut world, entity, _| {
                let placeholder = world
                    .entity(entity)
                    .get::<CameraPlaceholder>()
                    .unwrap()
                    .clone();

                world
                    .commands()
                    .entity(entity)
                    .remove::<CameraPlaceholder>()
                    .insert(Camera3dBundle {
                        transform: placeholder.transform,
                        ..Default::default()
                    });
            });
        world
            .register_component_hooks::<PointLightPlaceholder>()
            .on_add(|mut world, entity, _| {
                let PointLightPlaceholder {
                    point_light,
                    transform,
                } = world
                    .entity(entity)
                    .get::<PointLightPlaceholder>()
                    .unwrap()
                    .clone();

                world
                    .commands()
                    .entity(entity)
                    .remove::<PointLightPlaceholder>()
                    .insert(PointLightBundle {
                        transform,
                        point_light,
                        ..Default::default()
                    });
            });
        world
            .register_component_hooks::<PbrPlaceholder>()
            .on_add(|mut world, entity, _| {
                let placeholder = world
                    .entity(entity)
                    .get::<PbrPlaceholder>()
                    .unwrap()
                    .clone();

                let mut meshes = world.resource_mut::<Assets<Mesh>>();
                let mesh = meshes.add(placeholder.mesh);
                let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
                let material = materials.add(placeholder.material);

                world
                    .commands()
                    .entity(entity)
                    .remove::<PbrPlaceholder>()
                    .insert(PbrBundle {
                        transform: placeholder.transform.clone(),
                        mesh,
                        material,
                        ..Default::default()
                    });
            });
    }
}

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct CameraPlaceholder {
    pub transform: Transform,
}

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct PointLightPlaceholder {
    pub point_light: PointLight,
    pub transform: Transform,
}

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct PbrPlaceholder {
    pub material: MaterialPlaceholder,
    pub mesh: MeshPlaceholder,
    pub transform: Transform,
}

#[derive(Clone, Reflect)]
pub enum MeshPlaceholder {
    Cuboid(Cuboid),
    Circle(Circle),
}

impl Default for MeshPlaceholder {
    fn default() -> Self {
        MeshPlaceholder::Cuboid(Cuboid::new(1.0, 1.0, 1.0))
    }
}

impl Into<MeshPlaceholder> for Circle {
    fn into(self) -> MeshPlaceholder {
        MeshPlaceholder::Circle(self)
    }
}
impl Into<MeshPlaceholder> for Cuboid {
    fn into(self) -> MeshPlaceholder {
        MeshPlaceholder::Cuboid(self)
    }
}

impl Into<Mesh> for MeshPlaceholder {
    fn into(self) -> Mesh {
        match self {
            MeshPlaceholder::Circle(circle) => circle.into(),
            MeshPlaceholder::Cuboid(cuboid) => cuboid.into(),
        }
    }
}

#[derive(Clone, Reflect)]
pub enum MaterialPlaceholder {
    Color(Color),
}
impl Default for MaterialPlaceholder {
    fn default() -> Self {
        MaterialPlaceholder::Color(Color::WHITE)
    }
}

impl Into<MaterialPlaceholder> for Color {
    fn into(self) -> MaterialPlaceholder {
        MaterialPlaceholder::Color(self)
    }
}

impl Into<StandardMaterial> for MaterialPlaceholder {
    fn into(self) -> StandardMaterial {
        match self {
            MaterialPlaceholder::Color(color) => color.into(),
        }
    }
}
