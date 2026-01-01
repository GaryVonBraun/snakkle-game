use bevy::prelude::*;

pub mod components;
mod systems;

use crate::AppState;
use crate::game::SimulationState;
use crate::game::player::PlayerSet;
use crate::game::player::messages::AteFood;
use systems::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SegmentSet {
    Movement,
}
pub struct SegmentPlugin;

impl Plugin for SegmentPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, SegmentSet::Movement.before(PlayerSet::Movement));
        app.add_systems(OnEnter(AppState::Game), setup_segments);
        app.add_systems(
            Update,
            move_segments
                .in_set(SegmentSet::Movement)
                .before(PlayerSet::Movement)
                .run_if(in_state(AppState::Game)).run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(Update, grow_new_segment.run_if(on_message::<AteFood>));
        app.add_systems(OnExit(AppState::Game), despawn_segments);
    }
}
