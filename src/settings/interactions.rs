use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum ScreenModeInteraction {
    Windowed,
    Borderless,
    FullScreen,
}

#[derive(Component, Debug)]
pub enum SettingsNavigationInteraction {
    Back,
}
