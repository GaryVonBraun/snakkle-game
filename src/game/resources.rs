use bevy::prelude::*;

#[derive(Resource)]
pub struct SnakeTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct GameScore {
    pub current_score: i32,
}
