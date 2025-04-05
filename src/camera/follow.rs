use bevy::prelude::*;
use crate::entities::player::Player;
use crate::camera::setup::CameraRoot;

pub fn follow_player_camera(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut camera_root_query: Query<&mut Transform, (With<CameraRoot>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let player_position = player_transform.translation();

    for mut transform in &mut camera_root_query {
        transform.translation = player_position;
        // Optional: You could also rotate here for freelook or head bob
    }
}
