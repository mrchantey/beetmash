use crate::prelude::*;
use bevy::prelude::*;

pub fn spawn_simple_environment(mut commands: Commands) {
    // light
    commands.spawn(PointLightPlaceholder {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(CameraPlaceholder {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn spawn_simple_scene(mut commands: Commands) {
    // circular base
    commands.spawn(PbrPlaceholder {
        mesh: Circle::new(4.0).into(),
        material: Color::WHITE.into(),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    });
    // cube
    commands.spawn(PbrPlaceholder {
        mesh: Cuboid::new(1.0, 1.0, 1.0).into(),
        material: Color::srgb_u8(124, 144, 255).into(),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}
