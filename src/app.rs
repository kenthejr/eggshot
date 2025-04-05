use bevy::prelude::*;

// Use `crate::` because `camera` and `scene` are in the top-level `src/`, not in `app/`
use crate::camera::CameraPlugin;
use crate::scene::ScenePlugin;
use crate::systems::movement::{player_movement_input, apply_gravity, player_jump};

pub fn build_app() -> App {

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins((
            CameraPlugin,
            ScenePlugin,
        ))
        .add_systems(Update, (
            player_movement_input,
            apply_gravity,
            player_jump,
        ));

    app
}
