mod setup;
pub use setup::spawn_scene;

use bevy::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_scene);
    }
}
