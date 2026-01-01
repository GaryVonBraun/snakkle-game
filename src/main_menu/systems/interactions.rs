use bevy::prelude::*;

use crate::{
    AppState,
    main_menu::components::*, styling::{PRIMARY_BUTTON_COLOR, PRIMARY_BUTTON_COLOR_HOVERED, PRIMARY_BUTTON_COLOR_PRESSED},
};

pub fn play_button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match interaction {
            Interaction::Hovered => {
                *background_color = PRIMARY_BUTTON_COLOR_HOVERED.into();
            }
            Interaction::Pressed => {
                *background_color = PRIMARY_BUTTON_COLOR_PRESSED.into();
                next_state.set(AppState::Game);
            }
            Interaction::None => {
                *background_color = PRIMARY_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn quit_button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_message_writer: MessageWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match interaction {
            Interaction::Hovered => {
                *background_color = PRIMARY_BUTTON_COLOR_HOVERED.into();
            }
            Interaction::Pressed => {
                *background_color = PRIMARY_BUTTON_COLOR_PRESSED.into();
                app_exit_message_writer.write(AppExit::Success);

            }
            Interaction::None => {
                *background_color = PRIMARY_BUTTON_COLOR.into();
            }
        }
    }
}
