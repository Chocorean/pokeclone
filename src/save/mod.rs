mod components;
mod systems;

use bevy::prelude::*;
pub(crate) use components::Save;
pub use systems::*;

use crate::{AppState, ui::setup_game_ui};

const SAVE_PATH: &str = "assets/saves/save.json";

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            OnTransition {
                exited: AppState::MainMenu,
                entered: AppState::InGame,
            },
            apply_save.before(setup_game_ui),
        );
    }
}
