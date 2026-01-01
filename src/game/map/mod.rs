use bevy::prelude::*;

use crate::{AppState, game::map::systems::{despawn_map, setup_map}};

mod systems;
mod components;

pub struct GameMapPlugin;

impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_map);
        app.add_systems(OnExit(AppState::Game), despawn_map);
    }
}