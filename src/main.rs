use bevy::{prelude::*, window::WindowResolution};

mod game;
mod main_menu;
mod resources;
mod settings;
mod styling;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

use crate::settings::SettingsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Snakkle"),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resolution: WindowResolution::new(512, 512),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, startup_app)
        .add_systems(PreStartup, setup_resolution)
        .init_state::<AppState>()
        .add_systems(Update, set_app_state)
        .add_plugins(GamePlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(SettingsPlugin)
        .run();
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    MainMenu,
    Settings,
    Game,
    GameOver,
    #[default]
    Loading,
}