use bevy::color::palettes::basic::*;
use bevy::prelude::*;

use super::components::*;
use crate::game::player::messages::AteFood;

use crate::game::player::PLAYER_SIZE;
use crate::game::player::components::Player;
use crate::game::resources::SnakeTimer;
use crate::resources::Resolution;

pub fn setup_segments() {
    println!("Setting up segments");
}

pub fn despawn_segments(mut commands: Commands, segment_query: Query<(Entity, &Segment)>) {
    for (segment_entity, _segment) in segment_query.iter() {
        commands.entity(segment_entity).despawn();
    }
}

pub fn grow_new_segment(
    mut m_ate_food: MessageReader<AteFood>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    resolution: Res<Resolution>,
    segment_query: Query<(&Segment, &Transform)>,
    player_query: Query<(&Player, &Transform)>,
) {
    // previously i used player transform to spawn a segment in that location but after adding collision insta kills you
    let Ok((_player, _player_transform)) = player_query.single() else {
        return;
    };

    for _event in m_ate_food.read() {
        let segments: Vec<_> = segment_query.iter().collect();

        let translation: Vec3;
        match segments.last() {
            // THIS RIGHT HERE IS COPIUM, so im spawning a segment outside of view.
            // a better solution would maybe be spawning the segment after the player has moved, but this is simpler.
            None => translation = Vec3 { x: 10000., y: 10000., z: 10000. },
            Some((_segment, segment_transform)) => {
                translation = segment_transform.translation;
            }
        }

        commands.spawn((
            Transform::default()
                .with_scale(Vec3::splat(PLAYER_SIZE * resolution.pixel_ratio))
                .with_translation(translation),
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::from(GREEN))),
            Segment {
                index: segments.len() as i32,
            },
        ));
    }
}

pub fn move_segments(
    mut segment_query: Query<(&Segment, &mut Transform), Without<Player>>,
    snake_timer: Res<SnakeTimer>,
    player_query: Query<(&Player, &Transform)>,
) {
    // if the timer is not done we just return
    if !snake_timer.timer.is_finished() {
        return;
    }

    let Ok((mut _player, player_transform)) = player_query.single() else {
        return;
    };

    let mut segments: Vec<_> = segment_query.iter_mut().collect();
    segments.sort_by_key(|(segment, _t)| segment.index);
    let mut previous_position: Option<Vec3> = None;
    for (_segment, mut segment_transform) in segments {
        match previous_position {
            None => {
                previous_position = Some(segment_transform.translation);
                segment_transform.translation = player_transform.translation;
            }
            Some(new_position) => {
                previous_position = Some(segment_transform.translation);
                segment_transform.translation = new_position;
            }
        }
    }
}
