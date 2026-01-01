use bevy::prelude::*;

use crate::{
    AppState,
    game::ui::game_over::systems::{
        interactions::{main_menu_button_interaction, retry_button_interaction},
        layout::{despawn_game_over_menu, spawn_game_over_menu},
    },
};

mod components;
mod styles;
mod systems;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu);
        app.add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
        app.add_systems(
            Update,
            (retry_button_interaction, main_menu_button_interaction)
                .run_if(in_state(AppState::GameOver)),
        );
    }
}
