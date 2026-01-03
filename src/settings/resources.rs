use serde::{Serialize, Deserialize};
use bevy::prelude::*;

#[derive(Resource, Serialize, Deserialize)]
pub struct GameSettings {
    pub window_mode: WindowModeConfig,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum WindowModeConfig {
    Windowed,
    Borderless,
    FullScreen,
}