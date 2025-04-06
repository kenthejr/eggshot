use bevy::prelude::*;
use bevy::prelude::AppExtStates;
use bevy::app::AppExit;


// Use `crate::` because `camera` and `scene` are in the top-level `src/`, not in `app/`
use crate::camera::CameraPlugin;
use crate::scene::ScenePlugin;
use crate::systems::movement::{player_movement_input, apply_gravity, player_jump};
use crate::systems::mouse_look::{
    mouse_look_system,
    setup_cursor,
    unlock_cursor,
    LookAngles,
};
use crate::menu::MenuPlugin;
use crate::state::AppState;

pub fn build_app() -> App {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<AppState>() 
        .init_resource::<LookAngles>()
        .add_systems(Startup, setup_cursor)
        .add_plugins(ScenePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MenuPlugin)
        .add_systems(Update, (
            player_movement_input,
            apply_gravity,
            player_jump
        ))
        .add_systems(Update, (
            mouse_look_system,
            unlock_cursor
        ));

    app
}