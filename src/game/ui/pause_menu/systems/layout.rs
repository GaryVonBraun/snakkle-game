use crate::game::ui::pause_menu::components::*;
use crate::game::ui::pause_menu::*;
use crate::styling::primary_button_bundle;


pub fn spawn_pause_menu(mut commands: Commands) {
    build_pause_menu(&mut commands);
}

pub fn build_pause_menu(commands: &mut Commands) -> Entity {
    println!("Building pause menu");
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
            PauseMenu,
        ))
        .with_children(|parent| {
            // resume button
            parent
                .spawn((
                    primary_button_bundle(),
                    ResumeButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("resume".to_string()),
                        TextColor::from(TextColor::WHITE),
                    ));
                });
            // main menu button
            parent
                .spawn((
                    primary_button_bundle(),
                    ExitButton,
                ))
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

pub fn despawn_pause_menu(mut commands: Commands, pause_menu_query: Query<(Entity, &PauseMenu)>) {
    let Ok(pause_menu_entity) = pause_menu_query.single() else {
        return;
    };

    commands.entity(pause_menu_entity.0).despawn();
}
