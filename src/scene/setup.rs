use bevy::prelude::*;

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

    // Cube
    let cube_mesh = meshes.add(Mesh::from(Cuboid::default()));
    let cube_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.3, 0.3),
        ..default()
    });

    commands.spawn((
        Mesh3d(cube_mesh),
        MeshMaterial3d(cube_material),
        Transform::from_xyz(0.0, 0.5, 0.0),
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
