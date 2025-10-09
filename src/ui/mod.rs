use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::player::move_player_from_input;

use crate::AppState;

mod in_fight;
mod in_game;
mod index;
mod main_menu;
mod options_menu;
mod widgets;

use in_fight::*;
use in_game::*;
use index::*;
use main_menu::*;
use options_menu::*;
use widgets::*;

pub use in_game::setup_game_ui;
pub use main_menu::VirtualInput;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu_ui)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu_ui)
            .add_systems(OnEnter(AppState::OptionsMenu), setup_options_ui)
            .add_systems(OnExit(AppState::OptionsMenu), despawn_options_ui)
            .add_systems(
                OnTransition {
                    exited: AppState::MainMenu,
                    entered: AppState::InGame,
                },
                setup_game_ui,
            )
            .add_systems(
                OnTransition {
                    exited: AppState::InGame,
                    entered: AppState::MainMenu,
                },
                despawn_game_ui,
            )
            .add_systems(OnEnter(AppState::InFight), setup_fight_ui)
            .add_systems(OnExit(AppState::InFight), despawn_fight_ui)
            .add_systems(
                Update,
                (
                    handle_attack_button,
                    progress_attack_sequence,
                    handle_tame_button,
                    progress_tame_sequence,
                )
                    .run_if(in_state(AppState::InFight)),
            )
            .add_systems(
                Update,
                (refresh_team_ui, handle_game_ui_input).run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (
                    handle_hyperlinks,
                    handle_buttons,
                    handle_resume_game_button,
                    handle_new_game_button,
                    handle_options_button,
                    handle_return_button,
                    sync_slider_visuals,
                    handle_index_button,
                    handle_save_button,
                    handle_reset_save_button,
                    handle_new_log,
                    handle_quit_button,
                ),
            )
            .add_systems(OnEnter(AppState::Index), setup_index_ui)
            .add_systems(OnExit(AppState::Index), despawn_index_ui)
            .add_systems(
                Update,
                handle_index_ui_input.run_if(in_state(AppState::Index)),
            )
            .init_resource::<VirtualInput>();
        #[cfg(target_arch = "wasm32")]
        app.add_systems(Update, handle_wasm_gamepad.before(move_player_from_input));
    }
}
