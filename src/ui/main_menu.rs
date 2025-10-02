use bevy::prelude::*;

use crate::{
    AppState,
    dex::Dex,
    save::{Save, init_blank_save, load_save},
    ui::widgets::*,
};

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
        init_blank_save(commands, dex);
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

#[derive(Component)]
pub struct MainMenuUi;

pub(crate) fn setup_main_menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/mmc.otf");
    commands
        .spawn((
            MainMenuUi,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(12.0),
                ..default()
            })
            .with_children(|root| {
                root.spawn((
                    Text::new("PokeClone"),
                    TextFont {
                        font: font.clone(),
                        font_size: 40.,
                        ..default()
                    },
                ));
                root.spawn(hyperlink(
                    "Source code",
                    "https://github.com/Chocorean/pokeclone",
                    font.clone(),
                ));
                root.spawn(h_sep());

                // buttons
                if Save::exists() {
                    root.spawn(button("Resume Game", font.clone()))
                        .insert(ResumeGameButton);
                }
                root.spawn(button("New Game", font.clone()))
                    .insert(NewGameButton);
                root.spawn(button("Options", font.clone()))
                    .insert(OptionsButton);
            });
        });
}

pub(crate) fn despawn_main_menu_ui(
    mut commands: Commands,
    entity: Single<Entity, With<MainMenuUi>>,
) {
    commands.entity(*entity).despawn();
}
