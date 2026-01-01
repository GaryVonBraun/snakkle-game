use bevy::{color::palettes::css::BLACK, prelude::*};

use crate::{game::map::components::MapGrid, resources::Resolution};

pub const TILE_SIZE: f32 = 16.;
pub const TILE_MARGIN: f32 = 6.;

pub fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    resolution: Res<Resolution>,
) {
    for y in -10..10 {
        for x in -10..10 {
            commands.spawn((
                MapGrid,
                Transform::default()
                    .with_scale(Vec3::splat(TILE_SIZE * resolution.pixel_ratio))
                    .with_translation(Vec3 {
                        x: (TILE_SIZE + TILE_MARGIN) * x as f32,
                        y: (TILE_SIZE + TILE_MARGIN) * y as f32,
                        z: -10.,
                    }),
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(BLACK))),
            ));
        }
    }
}

pub fn despawn_map(map_grid_query: Query<Entity, With<MapGrid>>, mut commands: Commands) {
    for map_grid in map_grid_query.iter() {
        commands.entity(map_grid).despawn();
    }
}
