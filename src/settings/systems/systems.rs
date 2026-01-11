use std::fs;

use bevy::{prelude::*, window::{PrimaryWindow, WindowMode}};

use crate::settings::resources::{GameSettings, WindowModeConfig};

pub fn load_settings() -> GameSettings {
    let contents = fs::read_to_string("settings.toml");

    match contents {
        Ok(toml_content) => {
            toml::from_str(&toml_content).unwrap_or_else(|_| GameSettings::default())
        }
        Err(_) => GameSettings::default(),
    }
}

pub fn setup_settings(mut commands: Commands) {
    commands.insert_resource(load_settings());
}

pub fn apply_window_settings(
    settings: Res<GameSettings>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut window) = window_query.single_mut() else {
        return;
    };

    match settings.window_mode {
        WindowModeConfig::Windowed => {
            window.mode = WindowMode::Windowed;
        }
        WindowModeConfig::Borderless => {
            window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
        }
        WindowModeConfig::FullScreen => {
            window.mode = WindowMode::Fullscreen(
                MonitorSelection::Primary,
                VideoModeSelection::Current,
            );
        }
    }
}
