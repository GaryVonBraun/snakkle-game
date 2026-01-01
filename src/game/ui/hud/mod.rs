use bevy::prelude::*;

use crate::{
    AppState,
    game::{
        SimulationState,
        ui::hud::systems::{
            interactions::{pause_button_interaction, update_score_counter},
            layout::{despawn_hud, spawn_hud},
        },
    },
};

mod components;
mod styles;
mod systems;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_hud);
        app.add_systems(OnExit(AppState::Game), despawn_hud);
        app.add_systems(
            Update,
            (pause_button_interaction).run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            Update,
            (update_score_counter).run_if(in_state(SimulationState::Running)),
        );
    }
}
