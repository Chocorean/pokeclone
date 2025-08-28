use bevy::prelude::*;

pub mod fight;
pub mod game;
pub mod main_menu;
use bevy_egui::EguiPlugin;
use fight::*;
use game::*;
use main_menu::*;

use crate::{appstate::AppState, world::setup_world_ui};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default());
        app.add_systems(OnEnter(AppState::MainMenu), setup_menu)
            .add_systems(OnExit(AppState::MainMenu), hide_menu)
            .add_systems(Update, show_menu.run_if(in_state(AppState::MainMenu)))
            .add_systems(
                OnEnter(AppState::InGame),
                (setup_world_ui, setup_game_ui).chain(),
            )
            .add_systems(
                OnTransition {
                    exited: AppState::InGame,
                    entered: AppState::MainMenu,
                },
                hide_game_ui,
            )
            .add_systems(
                Update,
                (handle_button_interactions, handle_game_ui_input)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(Update, fight_ui.run_if(in_state(AppState::Fight)));
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
