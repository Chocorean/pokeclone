use bevy::prelude::*;
use bevy_ecs_ldtk::{GridCoords, LevelSelection};

use crate::{
    appstate::AppState,
    character::Player,
    save::Save,
    team::Team,
    ui::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    world::{WorldBundle, WorldTexture},
};

#[derive(Component)]
pub struct GameUI;

#[derive(Component)]
pub struct TeamUI;

#[derive(Component)]
pub struct WorldUI;

#[derive(Component)]
pub struct GameNode;

#[derive(Component)]
pub struct SaveButton;

pub fn setup_game_ui(
    mut commands: Commands,
    world_tex: Res<WorldTexture>,
    team: Res<Team>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            GameUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(Color::srgb_u8(99, 99, 99)), // dark gray
        ))
        .with_children(|parent| {
            // LEFT PANEL (80% width)
            parent
                .spawn((
                    Node {
                        width: Val::Percent(80.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BorderColor(Color::srgb_u8(55, 10, 150)),
                    BackgroundColor(Color::srgb_u8(150, 150, 150)), // lighter gray
                ))
                .with_children(|left| {
                    // TOP ROW
                    left.spawn((
                        Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            width: Val::Percent(100.0),
                            height: Val::Percent(15.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb_u8(0, 0, 200)),
                        BorderColor(Color::BLACK),
                    ))
                    .with_children(|top| {
                        top.spawn((
                            Button,
                            SaveButton,
                            Node {
                                width: Val::Px(50.),
                                height: Val::Px(35.),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(NORMAL_BUTTON),
                            children![(
                                Text::new("Save"),
                                TextFont {
                                    font_size: 10.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            )],
                        ));
                    });
                    // REMAINING AREA (fills the other 85%)
                    left.spawn(Node {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0, // take all remaining vertical space
                        justify_content: JustifyContent::Center, // center horizontally
                        align_items: AlignItems::Center, // center vertically
                        ..default()
                    })
                    .with_children(|center| {
                        // CENTERED BOX (80% × 80% of the remaining area)
                        center
                            .spawn(Node {
                                width: Val::Percent(80.0),
                                height: Val::Percent(80.0),
                                ..default()
                            })
                            .with_children(|center| {
                                center.spawn((
                                    WorldUI,
                                    ImageNode {
                                        image: world_tex.0.clone(),
                                        ..default()
                                    },
                                ));
                            });
                    });
                });
            parent
                .spawn((
                    // RIGHT PANEL (20% width)
                    TeamUI,
                    (Node {
                        width: Val::Percent(20.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    }),
                    BorderColor(Color::srgb_u8(150, 10, 55)),
                    BackgroundColor(Color::srgb_u8(20, 20, 20)), // darker gray
                ))
                .with_children(|col| {
                    for member in team.0.iter() {
                        col.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(15.0),
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::bottom(Val::Percent(3.)),
                                ..default()
                            },
                            BorderColor(Color::srgb_u8(230, 230, 230)),
                            BackgroundColor(Color::srgb_u8(50, 50, 50)),
                        ))
                        .with_children(|member_ui| {
                            let handle = asset_server.load(member.sprite());
                            member_ui.spawn((
                                Text::new(member.name()),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));
                            member_ui.spawn((
                                ImageNode {
                                    image: handle,
                                    ..default()
                                },
                                Node {
                                    width: Val::Px(64.),
                                    height: Val::Px(64.),
                                    ..default()
                                },
                            ));
                            member_ui.spawn((
                                Text::new(format!("HP: {}/{}", member.hp, member.max_hp())),
                                TextFont {
                                    font_size: 10.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));
                        });
                    }
                });
        });
}

/// Despawn all the game UI
pub fn hide_game_ui(
    mut commands: Commands,
    query: Query<Entity, Or<(With<GameUI>, With<WorldBundle>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Game UI specific input handling
/// Does not cover in-game actions like moving the player
pub fn handle_game_ui_input(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_q: Query<&GridCoords, With<Player>>,
    level_res: Option<Res<LevelSelection>>,
    mut exit: EventWriter<AppExit>,
    team: Res<Team>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        // next_state.set(AppState::MainMenu);
        exit.write(AppExit::Success);
    } else if keyboard_input.just_pressed(KeyCode::F1) {
        let level_id = match *level_res.unwrap() {
            LevelSelection::Indices(x) => x.level,
            _ => todo!("not supported"),
        };
        let coords = player_q.single().unwrap();
        Save::new(level_id as i32, *coords, team.clone());
    }
}

pub fn handle_button_interactions(
    save_q: Query<&Interaction, With<SaveButton>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    level_res: Option<Res<LevelSelection>>,
    player_q: Query<&GridCoords, With<Player>>,
    team: Res<Team>,
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
    let interaction = save_q.single().unwrap();
    if interaction == &Interaction::Pressed {
        let level_id = match *level_res.unwrap() {
            LevelSelection::Indices(x) => x.level,
            _ => todo!("not supported"),
        };
        let coords = player_q.single().unwrap();
        Save::new(level_id as i32, *coords, team.clone());
    }
}
