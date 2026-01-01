use bevy::prelude::*;

use crate::styling::PRIMARY_BUTTON_COLOR;


pub fn pause_button_bundle() -> impl Bundle {
    (
        Button,
        BackgroundColor(PRIMARY_BUTTON_COLOR),
        Node {
            height: Val::Px(20.),
            width: Val::Px(20.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    )
}
