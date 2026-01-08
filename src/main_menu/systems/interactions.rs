use bevy::prelude::*;

use crate::{
    AppState,
    main_menu::components::*,
    styling::{PRIMARY_BUTTON_COLOR, PRIMARY_BUTTON_COLOR_HOVERED, PRIMARY_BUTTON_COLOR_PRESSED},
};

pub fn main_menu_interactions(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &MainMenuInteractions),
        (Changed<Interaction>, With<MainMenuInteractions>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit_message_writer: MessageWriter<AppExit>,
) {
    for (interaction, mut background_color, &menu_interaction) in button_query.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                *background_color = PRIMARY_BUTTON_COLOR_HOVERED.into();
            }
            Interaction::None => {
                *background_color = PRIMARY_BUTTON_COLOR.into();
            }
            Interaction::Pressed => {
                *background_color = PRIMARY_BUTTON_COLOR_PRESSED.into();
                apply_menu_interactions(menu_interaction, &mut  next_state, &mut app_exit_message_writer);
            }
        }
    }
}

fn apply_menu_interactions(
    menu_interaction: MainMenuInteractions,
    next_state: &mut ResMut<NextState<AppState>>,
    app_exit_message_writer: &mut MessageWriter<AppExit>,
) {
    match menu_interaction {
        MainMenuInteractions::PlayButton => {
            next_state.set(AppState::Game);
        }
        MainMenuInteractions::SettingsButton => {
            next_state.set(AppState::Settings);
        }
        MainMenuInteractions::QuitButton => {
            app_exit_message_writer.write(AppExit::Success);
        }
    }
}
