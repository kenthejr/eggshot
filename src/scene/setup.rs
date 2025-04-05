use bevy::prelude::*;
use crate::entities::player::Player;
use crate::components::velocity::Velocity;

pub fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    let ground_mesh = meshes.add(Plane3d::default().mesh().size(10.0, 10.0));
    let ground_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.7, 0.2),
        ..default()
    });

    commands.spawn((
        Mesh3d(ground_mesh),
        MeshMaterial3d(ground_material),
        Transform::default(),
        GlobalTransform::default(),
    ));

    // Egg (stretched sphere)
    // Egg (stretched sphere)
    let egg_mesh = meshes.add(Mesh::from(Sphere { radius: 0.5 }));
    let egg_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.9, 0.85),
        perceptual_roughness: 0.7,
        ..default()
    });

    commands.spawn((
        Player,
        Velocity::default(),
        Mesh3d(egg_mesh),
        MeshMaterial3d(egg_material),
        Transform {
            translation: Vec3::new(0.0, 0.5, 0.0),
            scale: Vec3::new(0.8, 1.2, 0.8), // squashed X/Z, stretched Y = egg shape
            ..default()
        },
        GlobalTransform::default(),
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));
}
