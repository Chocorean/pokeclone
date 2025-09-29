use bevy::prelude::*;

use crate::{
    AppState,
    dex::Dex,
    save::{load_save, new_save},
};

// MAIN MENU

#[derive(Component)]
pub struct ResumeGameButton;

#[derive(Component)]
pub struct NewGameButton;

#[derive(Component)]
pub struct OptionsButton;

pub fn handle_resume_game_button(
    commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    button: Single<&Interaction, (Changed<Interaction>, With<ResumeGameButton>)>,
) {
    if matches!(*button, Interaction::Pressed) {
        load_save(commands);
        next_state.set(AppState::InGame);
    }
}

pub fn handle_new_game_button(
    commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    dex: Res<Dex>,
    button: Single<&Interaction, (Changed<Interaction>, With<NewGameButton>)>,
) {
    if matches!(*button, Interaction::Pressed) {
        new_save(commands, dex);
        next_state.set(AppState::InGame);
    }
}

pub fn handle_options_button(
    button: Single<&Interaction, (Changed<Interaction>, With<OptionsButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if matches!(*button, Interaction::Pressed) {
        next_state.set(AppState::OptionsMenu);
    }
}

// OPTIONS

#[derive(Component)]
pub struct ReturnButton;

pub fn handle_return_button(
    button: Single<&Interaction, (Changed<Interaction>, With<ReturnButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if matches!(*button, Interaction::Pressed) {
        next_state.set(AppState::MainMenu);
    }
}
