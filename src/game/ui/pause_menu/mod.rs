use bevy::prelude::*;

use crate::game::{SimulationState, ui::pause_menu::systems::{interactions::{main_menu_button_interaction, resume_button_interaction}, layout::*}};

mod systems;
mod styles;
mod components;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu);
        app.add_systems(OnExit(SimulationState::Paused), despawn_pause_menu);
        app.add_systems(Update, (resume_button_interaction, main_menu_button_interaction).run_if(in_state(SimulationState::Paused)));
    }}
