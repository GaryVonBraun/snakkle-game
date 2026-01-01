use bevy::prelude::*;

use crate::{AppState, game::{
    SimulationState,
    ui::pause_menu::{
        components::{ExitButton, ResumeButton},
    },
}, styling::{PRIMARY_BUTTON_COLOR, PRIMARY_BUTTON_COLOR_HOVERED, PRIMARY_BUTTON_COLOR_PRESSED}};

pub fn resume_button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>),
    >,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match interaction {
            Interaction::Hovered => {
                *background_color = PRIMARY_BUTTON_COLOR_HOVERED.into();
            }
            Interaction::Pressed => {
                *background_color = PRIMARY_BUTTON_COLOR_PRESSED.into();
                next_state.set(SimulationState::Running);
            }
            Interaction::None => {
                *background_color = PRIMARY_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn main_menu_button_interaction(
    mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<ExitButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut simulation_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match interaction {
            Interaction::Hovered => {
                *background_color = PRIMARY_BUTTON_COLOR_HOVERED.into();
            }
            Interaction::Pressed => {
                *background_color = PRIMARY_BUTTON_COLOR_PRESSED.into();
                simulation_state.set(SimulationState::Running);
                next_state.set(AppState::MainMenu);
            }
            Interaction::None => {
                *background_color = PRIMARY_BUTTON_COLOR.into();
            }
        }
    }
}
