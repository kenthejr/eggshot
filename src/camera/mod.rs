pub mod setup;
use bevy::prelude::*;

use crate::camera::setup::{spawn_camera, CameraSpawned};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CameraSpawned>() // ← Required to register the resource
            .add_systems(Update, spawn_camera);
    }
}

