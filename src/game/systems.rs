use bevy::{prelude::*};

use crate::game::{SimulationState, resources::GameScore};

pub fn setup_game(mut command: Commands) {
    command.spawn(Camera2d);
}

pub fn toggle_simulation_state(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<SimulationState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(SimulationState::Paused);
    }
}

pub fn initialize_game(mut commands: Commands){
    commands.insert_resource(GameScore{
        current_score: 0
    });
}