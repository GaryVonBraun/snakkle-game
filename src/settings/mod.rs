use bevy::prelude::*;

use crate::{
    AppState,
    settings::{interactions::*, layout::*, resources::GameSettings, systems::{systems::{apply_window_settings, setup_settings}, *}},
};
mod components;
mod resources;
mod styling;
mod systems;
  
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>();
        app.add_systems(Startup, (setup_settings, apply_window_settings).chain());
        app.add_systems(OnEnter(AppState::Settings), spawn_settings_menu);
        app.add_systems(OnExit(AppState::Settings), despawn_settings_menu);
        app.add_systems(
            Update,
            (screen_mode_interaction, settings_navigation_interaction)
                .run_if(in_state(AppState::Settings)),
        );
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            window_mode: resources::WindowModeConfig::Windowed,
        }
    }
}
