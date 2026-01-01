use bevy::prelude::*;

use super::resources::*;
use crate::AppState;

pub fn setup_resolution(mut commands: Commands, window_query: Query<&Window>) {
    println!("setup");
    let Ok(window) = window_query.single() else {
        return;
    };

    commands.insert_resource(Resolution {
        _screen_dimensions: vec2(window.width(), window.height()),
        pixel_ratio: 1.,
    });
}

pub fn set_app_state(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::KeyG) {
        next_state.set(AppState::Game);
    }
    if keys.just_pressed(KeyCode::KeyM) {
        next_state.set(AppState::MainMenu);
    }
}

pub fn startup_app(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::MainMenu);
}

