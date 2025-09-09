use bevy::prelude::*;

mod game;
mod index;
mod main_menu;

use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use game::*;
use main_menu::*;

use crate::AppState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default());
        app.add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin);
        app.add_systems(
            EguiPrimaryContextPass,
            (
                setup_main_menu_ui.run_if(in_state(AppState::MainMenu)),
                (setup_game_ui, handle_game_ui_input).run_if(in_state(AppState::InGame)),
            ),
        );
        app.add_systems(
            EguiPrimaryContextPass,
            setup_game_ui.run_if(in_state(AppState::InFight)),
        );
    }
}
