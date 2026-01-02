use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowMode},
};

use crate::{
    AppState, settings::interactions::{ScreenModeInteraction, SettingsNavigationInteraction}, styling::{PRIMARY_BUTTON_COLOR, PRIMARY_BUTTON_COLOR_HOVERED, PRIMARY_BUTTON_COLOR_PRESSED}
};

pub fn screen_mode_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &ScreenModeInteraction),
        (Changed<Interaction>, With<ScreenModeInteraction>),
    >,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    for (interaction, mut background_color, screen_mode_interaction) in button_query.iter_mut() {
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

                match screen_mode_interaction {
                    ScreenModeInteraction::Windowed => {
                        window.mode = WindowMode::Windowed;
                    }
                    ScreenModeInteraction::Borderless => {
                        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
                    }
                    ScreenModeInteraction::FullScreen => {
                        window.mode = WindowMode::Fullscreen(
                            MonitorSelection::Primary,
                            VideoModeSelection::Current,
                        );
                    }
                }
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
