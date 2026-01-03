use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct SettingsMenu;

#[derive(Component, Clone, Copy)]
pub enum WindowModeInteraction {
    Windowed,
    Borderless,
    FullScreen,
}

#[derive(Component, Debug)]
pub enum SettingsNavigationInteraction {
    Back,
}
