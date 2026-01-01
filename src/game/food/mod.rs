use bevy::prelude::*;

pub mod components;
mod systems;

use crate::{AppState, game::player::messages::AteFood};
use systems::*;

pub const FOOD_SIZE: f32 = 20.;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_food);
        app.add_systems(OnExit(AppState::Game), despawn_food);
        app.add_systems(Update, relocate_food.run_if(on_message::<AteFood>));
    }
}
