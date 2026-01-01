use crate::{
    game::ui::game_over::components::{GameOverMenu, MainMenuButton, RetryButton},
    styling::primary_button_bundle,
};
use bevy::prelude::*;

pub fn spawn_game_over_menu(mut commands: Commands) {
    build_game_over_menu(&mut commands);
}

pub fn build_game_over_menu(commands: &mut Commands) -> Entity {
    println!("Building game over menu");
    let pause_menu_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.),
                ..Default::default()
            },
            BackgroundColor::from(Color::linear_rgba(0.01, 0.01, 0.01, 0.5)),
            GameOverMenu,
        ))
        .with_children(|parent| {
            // title
            parent
                .spawn((Node { ..default() },))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("You dead".to_string()),
                        TextColor::from(TextColor::WHITE),
                    ));
                });
            // retry button
            parent
                .spawn((primary_button_bundle(), RetryButton))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("retry".to_string()),
                        TextColor::from(TextColor::WHITE),
                    ));
                });
            // main menu button
            parent
                .spawn((primary_button_bundle(), MainMenuButton))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("main menu".to_string()),
                        TextColor::from(TextColor::WHITE),
                    ));
                });
        })
        .id();
    pause_menu_entity
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    pause_menu_query: Query<(Entity, &GameOverMenu)>,
) {
    let Ok(pause_menu_entity) = pause_menu_query.single() else {
        return;
    };

    commands.entity(pause_menu_entity.0).despawn();
}
