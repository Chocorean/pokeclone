use bevy::prelude::*;

use crate::{
    appstate::AppState,
    save::Save,
    ui::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
};

#[derive(Component)]
pub struct ContinueButton;

#[derive(Component)]
pub struct NewGameButton;

#[derive(Component)]
pub struct OptionsButton;

#[derive(Component)]
pub struct MainMenuUI;

pub fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
            MainMenuUI,
            Node {
                // center button
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Pokeclone"),
                TextFont {
                    font_size: 50.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            parent.spawn(Node {
                margin: UiRect::bottom(Val::Px(130.)),
                ..default()
            });
            if Save::exists() {
                parent.spawn((
                    Button,
                    ContinueButton,
                    Node {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    children![(
                        Text::new("Continue"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    )],
                ));
            }
            parent.spawn((
                Button,
                NewGameButton,
                Node {
                    width: Val::Px(150.),
                    height: Val::Px(65.),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NORMAL_BUTTON),
                children![(
                    Text::new("New game"),
                    TextFont {
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                )],
            ));
            parent.spawn((
                Button,
                OptionsButton,
                Node {
                    width: Val::Px(150.),
                    height: Val::Px(65.),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NORMAL_BUTTON),
                children![(
                    Text::new("Options"),
                    TextFont {
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                )],
            ));
        });
}

pub fn hide_menu(mut commands: Commands, mut query: Query<Entity, With<MainMenuUI>>) {
    let entity = query.single_mut().unwrap();
    commands.entity(entity).despawn();
}

pub fn show_menu(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    continue_query: Query<&Interaction, With<ContinueButton>>,
    newgame_query: Query<&Interaction, With<NewGameButton>>,
    options_query: Query<&Interaction, With<OptionsButton>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
    let interaction = continue_query.single().unwrap();
    if interaction == &Interaction::Pressed {
        next_state.set(AppState::LoadGame);
    }
    let interaction = newgame_query.single().unwrap();
    if interaction == &Interaction::Pressed {
        next_state.set(AppState::InGame);
    }
    let interaction = options_query.single().unwrap();
    if interaction == &Interaction::Pressed {
        next_state.set(AppState::OptionsMenu);
    }
}
