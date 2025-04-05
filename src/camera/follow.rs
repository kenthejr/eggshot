use bevy::prelude::*;
use crate::entities::player::Player;

const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 4.0, 8.0);

pub fn follow_player_camera(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let player_position = player_transform.translation();

    for mut transform in &mut camera_query {
        transform.translation = player_position + CAMERA_OFFSET;
        transform.look_at(player_position, Vec3::Y);
    }
}
