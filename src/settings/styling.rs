use bevy::prelude::*;

use crate::styling::PRIMARY_BUTTON_COLOR;


pub fn settings_button_bundle() -> impl Bundle {
    (
        Button,
        BackgroundColor(PRIMARY_BUTTON_COLOR),
        Node {
            height: Val::Px(40.),
            width: Val::Px(130.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    )
}
