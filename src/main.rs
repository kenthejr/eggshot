mod app;
mod camera;
mod scene;
mod entities;
mod systems;
mod components;
mod state;
mod menu;

use bevy::prelude::*;
use crate::state::AppState;

fn main() {
    app::build_app().insert_state(AppState::Menu).run();
}
