use bevy::prelude::*;

#[derive(Resource)]
pub struct Resolution {
    pub _screen_dimensions: Vec2,
    pub pixel_ratio: f32,
}
