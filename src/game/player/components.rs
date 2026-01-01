use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Player {
    pub direction: Direction,
    pub previous_direction: Direction,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
