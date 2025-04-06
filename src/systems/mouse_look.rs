use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseMotion;

use crate::entities::player::Player;
use crate::camera::setup::CameraRoot;

const MOUSE_SENSITIVITY: f32 = 0.1;
const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01;

#[derive(Resource, Default)]
pub struct LookAngles {
    pub pitch: f32,
    pub yaw: f32,
}

pub fn setup_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor_options.visible = false;
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
}

pub fn mouse_look_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut look: ResMut<LookAngles>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_root_query: Query<&mut Transform, (With<CameraRoot>, Without<Player>)>,
) {
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        delta += event.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    look.pitch = (look.pitch - delta.y * MOUSE_SENSITIVITY * 0.01)
        .clamp(-PITCH_LIMIT, PITCH_LIMIT);
    look.yaw -= delta.x * MOUSE_SENSITIVITY * 0.01;

    // Apply yaw to player (horizontal turn)
    for mut transform in &mut player_query {
        transform.rotation = Quat::from_axis_angle(Vec3::Y, look.yaw);
    }

    // Apply pitch to camera root (vertical aim)
    for mut transform in &mut camera_root_query {
        transform.rotation = Quat::from_axis_angle(Vec3::X, look.pitch);
    }
}

pub fn unlock_cursor(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let mut window = windows.single_mut();
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}