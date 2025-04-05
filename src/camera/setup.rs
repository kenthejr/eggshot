use bevy::prelude::*;
use bevy::prelude::{Mesh3d, MeshMaterial3d};

#[derive(Component)]
pub struct CameraRoot;

pub fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera_root = commands
        .spawn((
            CameraRoot,
            Transform::from_xyz(0.0, 1.0, 0.0), // At egg head height
            GlobalTransform::default(),
        ))
        .id();

    let camera = commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 0.6, 0.0), // Slightly forward from root
            GlobalTransform::default(),
        ))
        .id();

    // Attach camera to root
    commands.entity(camera_root).add_child(camera);

    // Add a gun (placeholder cube) attached to the camera
    let gun_mesh = meshes.add(Mesh::from(Cuboid::new(0.2, 0.1, 0.4)));
    let gun_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.3, 0.3),
        ..default()
    });

    let gun = commands
        .spawn((
            Mesh3d(gun_mesh),
            MeshMaterial3d(gun_material),
            Transform::from_xyz(0.3, -0.3, -0.5),
            GlobalTransform::default(),
        ))
        .id();

    commands.entity(camera).add_child(gun);
}
