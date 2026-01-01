use bevy::prelude::*;

use crate::{
    game::{
        SimulationState,
        resources::GameScore,
        ui::hud::components::{PauseButton, ScoreCounter},
    },
    styling::{PRIMARY_BUTTON_COLOR, PRIMARY_BUTTON_COLOR_HOVERED, PRIMARY_BUTTON_COLOR_PRESSED},
};
pub fn pause_button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PauseButton>),
    >,
    mut simulation_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match interaction {
            Interaction::Hovered => {
                *background_color = PRIMARY_BUTTON_COLOR_HOVERED.into();
            }
            Interaction::Pressed => {
                *background_color = PRIMARY_BUTTON_COLOR_PRESSED.into();
                simulation_state.set(SimulationState::Paused);
            }
            Interaction::None => {
                *background_color = PRIMARY_BUTTON_COLOR.into();
            }
        }
    }
}
pub fn update_score_counter(
    mut score_counter_query: Query<&mut Text, With<ScoreCounter>>,
    game_score: Res<GameScore>,
) {
    if let Ok(mut text) = score_counter_query.single_mut() {
        text.0 = format!("score {}", game_score.current_score)
    }
}
