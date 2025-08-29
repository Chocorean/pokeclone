use bevy::prelude::*;

pub mod fight;
pub mod game;
pub mod main_menu;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use game::*;
use main_menu::*;

use crate::appstate::AppState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default());
        app.add_systems(
            EguiPrimaryContextPass,
            (
                setup_main_menu_ui.run_if(in_state(AppState::MainMenu)),
                setup_game_ui.run_if(in_state(AppState::InGame)),
            ),
        );
        // .add_systems(
        //     Update,
        //     (handle_button_interactions, handle_game_ui_input)
        //         .run_if(in_state(AppState::InGame)),
        // )
        // .add_systems(Update, fight_ui.run_if(in_state(AppState::InFight)))
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
