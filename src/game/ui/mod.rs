use bevy::prelude::*;

use crate::game::ui::game_over::GameOverMenuPlugin;
use crate::game::ui::pause_menu::PauseMenuPlugin;
use crate::game::ui::hud::HudPlugin;

mod hud;
mod pause_menu;
mod game_over;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(PauseMenuPlugin);
        app.add_plugins(HudPlugin);
        app.add_plugins(GameOverMenuPlugin);
    }
}