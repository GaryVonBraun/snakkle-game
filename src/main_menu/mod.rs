use crate::{AppState, main_menu::systems::interactions::*};
use bevy::prelude::*;
use systems::layout::*;
pub struct MainMenuPlugin;

mod components;
mod styles;
mod systems;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
        app.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
        app.add_systems(
            Update,
            main_menu_interactions.run_if(in_state(AppState::MainMenu)),
        );
    }
}
