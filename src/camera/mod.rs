pub mod setup;
mod follow;
pub use setup::spawn_camera;
pub use follow::follow_player_camera;

use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, follow_player_camera);
    }
}
