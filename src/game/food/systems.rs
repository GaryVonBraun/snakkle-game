use bevy::color::palettes::basic::*;
use bevy::prelude::*;
use rand::Rng;

use super::FOOD_SIZE;
use super::components::*;
use crate::game::player::PLAYER_SIZE;
use crate::game::player::components::Player;
use crate::game::segment::components::Segment;
use crate::resources::Resolution;

pub fn setup_food(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    resolution: Res<Resolution>,
) {
    let mut rng = rand::rng();
    let move_distance: f32 = (FOOD_SIZE + 2.) * resolution.pixel_ratio;

    commands.spawn((
        Transform::default()
            .with_translation(Vec3 {
                x: rng.random_range(-10..10) as f32 * move_distance,
                y: rng.random_range(-10..10) as f32 * move_distance,
                z: 0.,
            })
            .with_scale(Vec3::splat(FOOD_SIZE)),
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Food,
    ));
}

pub fn despawn_food(mut food_query: Query<(Entity, &Food)>, mut commands: Commands) {
    let Ok((food_entity, _food)) = food_query.single_mut() else {
        return;
    };

    commands.entity(food_entity).despawn();
}

pub fn relocate_food(
    mut food_query: Query<(&Food, &mut Transform), (Without<Player>, Without<Segment>)>,
    player_query: Query<(&Player, &Transform)>,
    segment_query: Query<(&Segment, &Transform)>,
    resolution: Res<Resolution>,
) {
    let Ok((_player, player_transform)) = player_query.single() else {
        return;
    };

    let mut rng = rand::rng();
    let move_distance: f32 = (FOOD_SIZE + 2.) * resolution.pixel_ratio;
    for (_food, mut food_transform) in food_query.iter_mut() {
        let mut valid_position = false;
        while valid_position == false {
            valid_position = true;
            
            food_transform.translation = Vec3 {
                x: rng.random_range(-10..10) as f32 * move_distance,
                y: rng.random_range(-10..10) as f32 * move_distance,
                z: 0.,
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
            ) < PLAYER_SIZE * 2.
            {
                valid_position = false;
            }
            for (_segment, segment_transform) in segment_query.iter() {
                if Vec2::distance(
                    Vec2 {
                        x: segment_transform.translation.x,
                        y: segment_transform.translation.y,
                    },
                    Vec2 {
                        x: food_transform.translation.x,
                        y: food_transform.translation.y,
                    },
                ) < FOOD_SIZE
                {
                    valid_position = false;
                    break;
                }
            }
        }
    }
}
