use bevy::prelude::*;

use crate::{AppState, settings::{layout::{despawn_settings_menu, spawn_settings_menu}, resources::Settings, systems::{screen_mode_interaction, settings_navigation_interaction}}};
mod resources;
mod styling;
mod interactions;
mod layout;
mod components;
mod systems;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Settings>();
        app.add_systems(OnEnter(AppState::Settings), spawn_settings_menu);
        app.add_systems(OnExit(AppState::Settings), despawn_settings_menu);
        app.add_systems(Update, (screen_mode_interaction, settings_navigation_interaction).run_if(in_state(AppState::Settings)));

    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings { screen_mode: resources::ScreenMode::Windowed }
    }
}