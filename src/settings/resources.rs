use bevy::prelude::*;

#[derive(Resource)]
pub struct Settings {
    pub screen_mode: ScreenMode,
}

pub enum ScreenMode {
    Windowed,
    Borderless,
    FullScreen,
}