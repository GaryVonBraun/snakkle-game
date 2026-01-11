use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Player {
    pub direction: Direction,
    pub previous_direction: Direction,
    pub previous_translation: Vec3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
