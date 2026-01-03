use std::fs;

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowMode},
};

use crate::{
    AppState,
    settings::{
        components::*,
        resources::{GameSettings, WindowModeConfig},
    },
    styling::{PRIMARY_BUTTON_COLOR, PRIMARY_BUTTON_COLOR_HOVERED, PRIMARY_BUTTON_COLOR_PRESSED},
};



pub fn screen_mode_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &WindowModeInteraction),
        (Changed<Interaction>, With<WindowModeInteraction>),
    >,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut game_settings: ResMut<GameSettings>,
) {
    for (interaction, mut background_color, window_mode_interaction) in button_query.iter_mut() {
        match interaction {
            Interaction::None => {
                *background_color = PRIMARY_BUTTON_COLOR.into();
            }
            Interaction::Hovered => {
                *background_color = PRIMARY_BUTTON_COLOR_HOVERED.into();
            }
            Interaction::Pressed => {
                *background_color = PRIMARY_BUTTON_COLOR_PRESSED.into();
                let Ok(mut window) = window_query.single_mut() else {
                    return;
                };

                match window_mode_interaction {
                    WindowModeInteraction::Windowed => {
                        window.mode = WindowMode::Windowed;
                        game_settings.window_mode = WindowModeConfig::Windowed;
                    }
                    WindowModeInteraction::Borderless => {
                        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
                        game_settings.window_mode = WindowModeConfig::Borderless;
                    }
                    WindowModeInteraction::FullScreen => {
                        window.mode = WindowMode::Fullscreen(
                            MonitorSelection::Primary,
                            VideoModeSelection::Current,
                        );
                        game_settings.window_mode = WindowModeConfig::FullScreen;
                    }
                }
                save_settings(&game_settings);
            }
        }
    }
}

pub fn settings_navigation_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SettingsNavigationInteraction>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match interaction {
            Interaction::None => {
                *background_color = PRIMARY_BUTTON_COLOR.into();
            }
            Interaction::Hovered => {
                *background_color = PRIMARY_BUTTON_COLOR_HOVERED.into();
            }
            Interaction::Pressed => {
                *background_color = PRIMARY_BUTTON_COLOR_PRESSED.into();
                next_state.set(AppState::MainMenu);
            }
        }
    }
}

fn save_settings(settings: &GameSettings) {
    if let Ok(toml) = toml::to_string_pretty(settings) {
        let _ = fs::write("settings.toml", toml);
    }
}