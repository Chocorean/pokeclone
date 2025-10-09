use bevy::prelude::*;
use bevy_easy_gif::GifNode;

use crate::{AppState, dex::Dex, ui::VirtualInput};

#[derive(Component)]
pub struct DyeButton;

#[derive(Component)]
pub(crate) struct IndexUi;

pub fn setup_index_ui(mut commands: Commands, asset_server: Res<AssetServer>, dex: Res<Dex>) {
    let image = asset_server.load("textures/index/dye.png");
    let font = asset_server.load("fonts/mmc.otf");

    commands
        .spawn((
            IndexUi,
            Node {
                top: Val::Percent(5.),
                left: Val::Percent(5.),
                width: Val::Percent(90.),
                height: Val::Px(630.), // looks the same on wasm too
                ..default()
            },
            BackgroundColor(Color::srgba_u8(0, 0, 0, 240)),
        ))
        .with_children(|ui| {
            ui.spawn((
                Button,
                DyeButton,
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(50.),
                    bottom: Val::Px(50.),
                    width: Val::Px(64.),
                    height: Val::Px(64.),
                    ..default()
                },
                ImageNode { image, ..default() },
            ));
            // random creature layout here.
            ui.spawn(random_index_entry(&dex, font));
        });
}

#[derive(Component)]
pub struct IndexContent;

pub fn random_index_entry(dex: &Dex, font: Handle<Font>) -> impl Bundle {
    let (data, handle) = dex.random();
    let attrs = dex.species[data.species_id].attributes.clone();
    (
        IndexContent,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        children![
            // image top left
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(25.),
                    left: Val::Percent(5.),
                    width: Val::Percent(30.),
                    height: Val::Percent(30.),
                    ..default()
                },
                GifNode { handle }
            ),
            // species
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(60.),
                    left: Val::Percent(13.),
                    ..default()
                },
                Text::new(format!(
                    "Species: {}",
                    dex.species[data.species_id].name.clone()
                )),
                TextFont {
                    font: font.clone(),
                    font_size: 12.,
                    ..default()
                }
            ),
            // Name
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(85.),
                    left: Val::Percent(35.),
                    ..default()
                },
                Text::new(data.name),
                TextFont {
                    font: font.clone(),
                    font_size: 32.,
                    ..default()
                }
            ),
            // stats title
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(35.),
                    left: Val::Percent(45.),
                    ..default()
                },
                Text::new("Stats"),
                TextFont {
                    font: font.clone(),
                    font_size: 24.,
                    ..default()
                }
            ),
            // stats tab
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(45.),
                    left: Val::Percent(45.),
                    ..default()
                },
                Text::new(
                    data.stats
                        .into_iter()
                        .map(|(id, v)| format!("+ {id}: {v}"))
                        .reduce(|a, b| format!("{a}\n{b}"))
                        .unwrap()
                ),
                TextFont {
                    font: font.clone(),
                    font_size: 14.,
                    ..default()
                }
            ),
            // attributes title
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(35.),
                    left: Val::Percent(75.),
                    ..default()
                },
                Text::new("Attributes"),
                TextFont {
                    font: font.clone(),
                    font_size: 24.,
                    ..default()
                }
            ),
            // attributes tab
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(45.),
                    left: Val::Percent(75.),
                    ..default()
                },
                Text::new(
                    attrs
                        .iter()
                        .map(|attr| format!("+ {:?}", attr))
                        .reduce(|a, b| format!("{a}\n{b}"))
                        .unwrap()
                ),
                TextFont {
                    font: font.clone(),
                    font_size: 14.,
                    ..default()
                }
            )
        ],
    )
}

pub fn despawn_index_ui(mut commands: Commands, entity: Single<Entity, With<IndexUi>>) {
    commands.entity(*entity).despawn();
}

pub fn handle_index_ui_input(
    input: Res<ButtonInput<KeyCode>>,
    virtual_input: Res<VirtualInput>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if input.just_released(KeyCode::Escape) | virtual_input.no {
        next_state.set(AppState::InGame);
    } else if input.just_released(KeyCode::Enter) | virtual_input.ok {
        // randomize
    }
}

pub fn handle_dye_hover(
    asset_server: Res<AssetServer>,
    mut button: Single<(&Interaction, &mut ImageNode), (Changed<Interaction>, With<DyeButton>)>,
) {
    if matches!(button.0, Interaction::None) {
        button.1.image = asset_server.load("textures/index/dye.png");
    } else {
        button.1.image = asset_server.load("textures/index/dye_hover.png");
    }
}

pub fn handle_dye_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dex: Res<Dex>,
    // fire once per press
    btn: Single<&Interaction, (Changed<Interaction>, With<DyeButton>)>,
    content: Single<Entity, With<IndexContent>>,
    ui: Single<Entity, With<IndexUi>>,
) {
    if !matches!(*btn, Interaction::Pressed) {
        return;
    }

    commands.entity(*content).despawn();
    let font = asset_server.load("fonts/mmc.otf");

    commands.entity(*ui).with_children(|ui| {
        ui.spawn(random_index_entry(&dex, font));
    });
}
