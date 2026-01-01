use bevy::prelude::*;

pub mod components;
pub mod messages;
mod systems;

use crate::{AppState, game::SimulationState};
use messages::*;
use systems::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSet {
    Movement,
}

pub const PLAYER_SIZE: f32 = 20.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, PlayerSet::Movement);
        app.add_message::<AteFood>();
        app.add_systems(OnEnter(AppState::Game), setup_player);
        app.add_systems(OnExit(AppState::Game), despawn_player);
        app.add_systems(
            Update,
            player_input
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            Update,
            (check_food_collision, check_segment_collision, check_border_collision)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)).after(PlayerSet::Movement),
        );
        app.add_systems(
            Update,
            move_player
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
                .in_set(PlayerSet::Movement),
        );
    }
}
