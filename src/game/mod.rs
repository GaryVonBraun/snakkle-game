use bevy::prelude::*;

mod food;
pub mod player;
mod resources;
mod segment;
mod systems;
mod ui;
mod map;

use crate::{AppState, game::{map::GameMapPlugin, ui::GameUIPlugin}};
use food::FoodPlugin;
use player::PlayerPlugin;
use resources::*;
use segment::SegmentPlugin;
use systems::*;

pub struct GamePlugin;

const SNAKE_MOVE_TIME: f32 = 0.20;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game);
        app.init_resource::<SnakeTimer>();
        app.init_resource::<GameScore>();
        app.init_state::<SimulationState>();
        app.add_plugins(PlayerPlugin);
        app.add_plugins(SegmentPlugin);
        app.add_plugins(FoodPlugin);
        app.add_plugins(GameUIPlugin);
        app.add_plugins(GameMapPlugin);
        app.add_systems(OnEnter(AppState::Game), initialize_game);
        app.add_systems(Update, tick_snake_timer.run_if(in_state(AppState::Game)).run_if(in_state(SimulationState::Running)));
        app.add_systems(Update, toggle_simulation_state.run_if(in_state(AppState::Game)));
    }
}

impl Default for SnakeTimer {
    fn default() -> Self {
        SnakeTimer {
            timer: Timer::from_seconds(SNAKE_MOVE_TIME, TimerMode::Repeating),
        }
    }
}

impl Default for GameScore {
    fn default() -> Self {
        GameScore { current_score: 0 }
    }
}

pub fn tick_snake_timer(mut snake_timer: ResMut<SnakeTimer>, time: Res<Time>) {
    snake_timer.timer.tick(time.delta());
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
