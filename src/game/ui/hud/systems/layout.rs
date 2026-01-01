use crate::game::ui::hud::{
    components::{HUD, PauseButton, ScoreCounter},
    styles::pause_button_bundle,
};
use bevy::prelude::*;

pub fn spawn_hud(mut commands: Commands) {
    build_hud(&mut commands);
}

pub fn build_hud(commands: &mut Commands) -> Entity {
    println!("Building Hud");
    let main_menu_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            BackgroundColor::from(Color::linear_rgba(0.01, 0.01, 0.01, 0.5)),
            HUD,
        ))
        .with_children(|parent| {
            // score_counter
            parent.spawn(Node { ..default() }).with_children(|parent| {
                parent.spawn((Text::new("score 0"), TextColor::WHITE, ScoreCounter));
            });

            // pause button
            parent
                .spawn((pause_button_bundle(), PauseButton))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("<".to_string()),
                        TextColor::from(TextColor::WHITE),
                        
                    ));
                });
        })
        .id();
    main_menu_entity
}

pub fn despawn_hud(mut commands: Commands, main_menu_query: Query<(Entity, &HUD)>) {
    let Ok(main_menu_entity) = main_menu_query.single() else {
        return;
    };

    commands.entity(main_menu_entity.0).despawn();
}
