use bevy::prelude::*;

pub const PRIMARY_BUTTON_COLOR: Color = Color::linear_rgb(0.05, 0.05, 0.05);
pub const PRIMARY_BUTTON_COLOR_HOVERED: Color = Color::linear_rgb(0.07, 0.07, 0.07);
pub const PRIMARY_BUTTON_COLOR_PRESSED: Color = Color::linear_rgb(0.02, 0.02, 0.02);

pub fn primary_button_bundle() -> impl Bundle {
    (
        Button,
        BackgroundColor(PRIMARY_BUTTON_COLOR),
        Node {
            height: Val::Px(80.),
            width: Val::Px(200.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    )
}
