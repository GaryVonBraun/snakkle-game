use crate::{
    settings::{
        components::SettingsMenu,
        interactions::{ScreenModeInteraction, SettingsNavigationInteraction},
        styling::settings_button_bundle,
    },
    styling::primary_button_bundle,
};

use bevy::prelude::*;
pub fn spawn_settings_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_settings_menu(&mut commands, &asset_server);
}

pub fn build_settings_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    println!("Building settings menu");
    let main_menu_entity = commands
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
            SettingsMenu,
        ))
        .with_children(|parent| {
            // title
            parent
                .spawn(
                    (Node {
                        width: Val::Px(300.),
                        height: Val::Px(120.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(20.),
                        ..Default::default()
                    }),
                )
                .with_children(|parent| {
                    parent.spawn((Text::new("Settings".to_string()), TextColor::WHITE));
                });

            // screen modes
            parent
                .spawn(Node {
                    column_gap: Val::Px(10.),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((settings_button_bundle(), ScreenModeInteraction::Windowed))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Windowed".to_string()),
                                TextColor::from(TextColor::WHITE),
                            ));
                        });
                    parent
                        .spawn((settings_button_bundle(), ScreenModeInteraction::Borderless))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Borderless".to_string()),
                                TextColor::from(TextColor::WHITE),
                            ));
                        });
                    parent
                        .spawn((settings_button_bundle(), ScreenModeInteraction::FullScreen))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Fullscreen".to_string()),
                                TextColor::from(TextColor::WHITE),
                            ));
                        });
                });
            // back button
            parent
                .spawn((primary_button_bundle(), SettingsNavigationInteraction::Back))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("back".to_string()),
                        TextColor::from(TextColor::WHITE),
                    ));
                });
        })
        .id();
    main_menu_entity
}

pub fn despawn_settings_menu(
    mut commands: Commands,
    main_menu_query: Query<(Entity, &SettingsMenu)>,
) {
    let Ok(main_menu_entity) = main_menu_query.single() else {
        return;
    };

    commands.entity(main_menu_entity.0).despawn();
}
