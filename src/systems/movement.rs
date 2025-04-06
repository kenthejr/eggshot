use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;

use crate::entities::player::Player;
use crate::components::velocity::Velocity;

const MOVE_SPEED: f32 = 5.0;
const GRAVITY: f32 = -9.8;
const JUMP_FORCE: f32 = 6.5;

pub fn player_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();

    for (mut transform, _velocity) in &mut query {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            // Transform the movement direction by the player's rotation
            let movement = transform.rotation * direction;
            // Apply horizontal movement (XZ only)
            transform.translation += Vec3::new(movement.x, 0.0, movement.z) * MOVE_SPEED * delta;
        }
    }
}

pub fn apply_gravity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let delta = time.delta().as_secs_f32();

    for (mut transform, mut velocity) in &mut query {
        velocity.linvel.y += GRAVITY * delta;
        transform.translation += velocity.linvel * delta;

        // Clamp to ground (simple ground collision)
        if transform.translation.y <= 0.5 {
            transform.translation.y = 0.5;
            velocity.linvel.y = 0.0;
        }
    }
}

pub fn player_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &mut Velocity), With<Player>>,
) {
    for (transform, mut velocity) in &mut query {
        if transform.translation.y <= 0.51 && keyboard_input.just_pressed(KeyCode::Space) {
            velocity.linvel.y = JUMP_FORCE;
        }
    }
}
