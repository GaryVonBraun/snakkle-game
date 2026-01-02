use crate::main_menu::components::*;
use crate::main_menu::styles::main_menu_img_bundle;
use crate::styling::primary_button_bundle;
use bevy::prelude::*;
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    println!("Building main menu");
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
            MainMenu,
        ))
        .with_children(|parent| {
            // title
            parent
                .spawn((Node {
                    width: Val::Px(300.),
                    height: Val::Px(120.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(20.),
                    ..Default::default()
                },))
                .with_children(|parent| {
                    // img 1
                    parent.spawn(main_menu_img_bundle(asset_server.load("snakelogo.png")));
                    // title
                    parent.spawn((Text::new("Snakkle".to_string()), TextColor::WHITE));
                    // img
                    parent.spawn(main_menu_img_bundle(asset_server.load("snakelogo.png")));
                });
            // play button
            parent
                .spawn((
                    primary_button_bundle(),
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("play".to_string()),
                        TextColor::from(TextColor::WHITE),
                    ));
                });
            // quit button
            parent
                .spawn((
                    primary_button_bundle(),
                    SettingsButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("settings".to_string()),
                        TextColor::from(TextColor::WHITE),
                    ));
                });
            parent
                .spawn((
                    primary_button_bundle(),
                    QuitButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("quit".to_string()),
                        TextColor::from(TextColor::WHITE),
                    ));
                });
        })
        .id();
    main_menu_entity
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<(Entity, &MainMenu)>) {
    let Ok(main_menu_entity) = main_menu_query.single() else {
        return;
    };

    commands.entity(main_menu_entity.0).despawn();
}
