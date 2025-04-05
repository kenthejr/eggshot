use bevy::prelude::*;

// Use `crate::` because `camera` and `scene` are in the top-level `src/`, not in `app/`
use crate::camera::CameraPlugin;
use crate::scene::ScenePlugin;

pub fn build_app() -> App {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins((
            CameraPlugin,
            ScenePlugin,
        ));

    app
}
