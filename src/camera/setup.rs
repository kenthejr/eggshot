use bevy::prelude::*;
use bevy::math::primitives::Cuboid;

use crate::entities::player::Player;

#[derive(Component)]
pub struct CameraRoot;

#[derive(Resource, Default)]
pub struct CameraSpawned(pub bool);

pub fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<Entity, With<Player>>,
    mut camera_state: ResMut<CameraSpawned>,
) {
    if camera_state.0 {
        return;
    }

    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    // Parent the entire camera rig under the Player
    commands.entity(player_entity).with_children(|player| {
        // CameraRoot handles pitch
        player.spawn((
            CameraRoot,
            Transform::from_xyz(0.0, 1.0, 0.0), // head height
            GlobalTransform::default(),
        ))
        .with_children(|camera_root| {
            // Camera3d inside the root
            camera_root.spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 0.6, 0.0), // forward camera offset
                GlobalTransform::default(),
            ))
            .with_children(|camera| {
                // Gun model attached to camera
                camera.spawn((
                    Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.2, 0.1, 0.4)))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(0.3, 0.3, 0.3),
                        ..default()
                    })),
                    Transform::from_xyz(0.3, -0.3, -0.5),
                    GlobalTransform::default(),
                ));
            });
        });
    });

    camera_state.0 = true;
}
