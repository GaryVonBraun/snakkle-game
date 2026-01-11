use bevy::color::palettes::basic::*;
use bevy::prelude::*;

use super::PLAYER_SIZE;
use super::components::*;
use super::messages::*;
use crate::AppState;
use crate::game::food::components::Food;
use crate::game::resources::GameScore;
use crate::game::resources::SnakeTimer;
use crate::game::segment::components::Segment;
use crate::resources::Resolution;

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    resolution: Res<Resolution>,
) {
    println!("setting up player");
    commands.spawn((
        Transform::default().with_scale(Vec3::splat(PLAYER_SIZE * resolution.pixel_ratio)),
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(GRAY))),
        Player {
            direction: Direction::Down,
            previous_direction: Direction::Down,
            previous_translation: vec3(0., 0., 0.),
        },
    ));
}

pub fn despawn_player(mut player_query: Query<(Entity, &Player)>, mut commands: Commands) {
    let Ok((player_entity, _player)) = player_query.single_mut() else {
        return;
    };

    commands.entity(player_entity).despawn();
}

pub fn player_input(mut player_query: Query<&mut Player>, keys: Res<ButtonInput<KeyCode>>) {
    let Ok(mut player) = player_query.single_mut() else {
        return;
    };

    if keys.pressed(KeyCode::KeyA) && player.previous_direction != Direction::Right {
        player.direction = Direction::Left;
    }
    if keys.pressed(KeyCode::KeyD) && player.previous_direction != Direction::Left {
        player.direction = Direction::Right;
    }
    if keys.pressed(KeyCode::KeyW) && player.previous_direction != Direction::Down {
        player.direction = Direction::Up;
    }
    if keys.pressed(KeyCode::KeyS) && player.previous_direction != Direction::Up {
        player.direction = Direction::Down;
    }
}

pub fn move_player(
    snake_timer: Res<SnakeTimer>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    resolution: Res<Resolution>,
) {
    let Ok((mut player, mut player_transform)) = player_query.single_mut() else {
        return;
    };
    if snake_timer.timer.is_finished() {
        player.previous_direction = player.direction;
        player.previous_translation = player_transform.translation;
        let move_distance: f32 = (PLAYER_SIZE + 2.) * resolution.pixel_ratio;

        match player.direction {
            Direction::Down => {
                player_transform.translation.y -= move_distance;
            }
            Direction::Up => {
                player_transform.translation.y += move_distance;
            }
            Direction::Right => {
                player_transform.translation.x += move_distance;
            }
            Direction::Left => {
                player_transform.translation.x -= move_distance;
            }
        }
    }
}

pub fn check_food_collision(
    player_query: Query<(&Player, &Transform)>,
    snake_timer: Res<SnakeTimer>,
    food_query: Query<(&Food, &Transform)>,
    game_score: Res<GameScore>,
    mut m_ate_food: MessageWriter<AteFood>,
    mut commands: Commands,
) {
    if !snake_timer.timer.is_finished() {
        return;
    };
    let Ok((_player, player_transform)) = player_query.single() else {
        return;
    };
    let Ok((_food, food_transform)) = food_query.single() else {
        return;
    };

    if Vec2::distance(
        Vec2 {
            x: player_transform.translation.x,
            y: player_transform.translation.y,
        },
        Vec2 {
            x: food_transform.translation.x,
            y: food_transform.translation.y,
        },
    ) < PLAYER_SIZE
    {
        println!("player collided with food");
        commands.insert_resource(GameScore {
            current_score: game_score.current_score + 1,
        });
        m_ate_food.write(AteFood);
    }
}

pub fn check_segment_collision(
    snake_timer: Res<SnakeTimer>,
    player_query: Query<(&Player, &Transform)>,
    segment_query: Query<(&Segment, &Transform)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !snake_timer.timer.is_finished() {
        return;
    };

    let Ok((_player, player_transform)) = player_query.single() else {
        return;
    };

    for (_segment, segment_transform) in segment_query.iter() {
        if Vec2::distance(
            Vec2 {
                x: player_transform.translation.x,
                y: player_transform.translation.y,
            },
            Vec2 {
                x: segment_transform.translation.x,
                y: segment_transform.translation.y,
            },
        ) < PLAYER_SIZE
        {
            println!("segment collision detected");
            next_state.set(AppState::GameOver);
        }
    }
}

const MAP_SIZE_NEG: f32 = 220.;
const MAP_SIZE_POS: f32 = 200.;

pub fn check_border_collision(
    snake_timer: Res<SnakeTimer>,
    player_query: Query<(&Player, &Transform)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !snake_timer.timer.is_finished() {
        return;
    };

    let Ok((_player, player_transform)) = player_query.single() else {
        return;
    };

    let player_translation = player_transform.translation;

    // rudimentary collision detection
    if player_translation.x > MAP_SIZE_POS
        || player_translation.x < -MAP_SIZE_NEG
        || player_translation.y > MAP_SIZE_POS
        || player_translation.y < -MAP_SIZE_NEG
    {
        println!("border collision detected");
        next_state.set(AppState::GameOver);
    }
}
