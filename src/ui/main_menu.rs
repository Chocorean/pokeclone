#[cfg(target_arch = "wasm32")]
use std::f32::consts::PI;

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

#[cfg(target_arch = "wasm32")]
#[derive(Component)]
pub enum Gamepad {
    Top,
    Left,
    Right,
    Bottom,
    Ok,
    No,
}

pub(crate) fn setup_main_menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/mmc.otf");
    commands
        .spawn((
            MainMenuUi,
            Node {
                width: Val::Percent(100.),
                height: Val::Px(700.),
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

    // virtual gamepad for wasm
    #[cfg(target_arch = "wasm32")]
    {
        let image = asset_server.load("textures/gamepad/arrow.png");
        commands
            .spawn(Node {
                position_type: PositionType::Absolute,
                top: Val::Px(690.),
                left: Val::Px(0.),
                ..default()
            })
            .with_children(|main| {
                // arrows
                for (rot, flip_y, x, y, kind) in [
                    (0., false, 0., 100., Gamepad::Top),
                    (PI / 2., true, 75., 25., Gamepad::Left),
                    (PI / 2., false, 75., 175., Gamepad::Right),
                    (0., true, 150., 100., Gamepad::Bottom),
                ] {
                    main.spawn((
                        Button,
                        kind,
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Px(128.),
                            height: Val::Px(128.),
                            top: Val::Px(x),
                            left: Val::Px(y),
                            ..default()
                        },
                        ImageNode {
                            image: image.clone(),
                            flip_y,
                            ..default()
                        },
                        Transform::from_rotation(Quat::from_axis_angle(Vec3::Z, rot)),
                    ));
                }
                // A/B buttons
                main.spawn((
                    Button,
                    Gamepad::Ok,
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(128.),
                        height: Val::Px(128.),
                        top: Val::Px(25.),
                        left: Val::Px(700.),
                        ..default()
                    },
                    ImageNode {
                        image: asset_server.load("textures/gamepad/button_ok.png"),
                        ..default()
                    },
                ));
                main.spawn((
                    Button,
                    Gamepad::No,
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(128.),
                        height: Val::Px(128.),
                        top: Val::Px(100.),
                        left: Val::Px(560.),
                        ..default()
                    },
                    ImageNode {
                        image: asset_server.load("textures/gamepad/button_no.png"),
                        ..default()
                    },
                ));
            });
    }
}

#[derive(Resource, Default)]
pub struct VirtualInput {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub ok: bool,
    pub no: bool,
}

#[cfg(target_arch = "wasm32")]
pub fn handle_wasm_gamepad(
    mut virt: ResMut<VirtualInput>,
    query: Query<(&Interaction, &Gamepad), (Changed<Interaction>, With<Gamepad>)>,
) {
    for (interaction, kind) in query {
        let pressed = matches!(*interaction, Interaction::Pressed);
        match kind {
            Gamepad::Left => virt.left = pressed,
            Gamepad::Right => virt.right = pressed,
            Gamepad::Top => virt.up = pressed,
            Gamepad::Bottom => virt.down = pressed,
            // for these two it's not great bc it stays pressed for quite some frames,
            // but Im done.
            Gamepad::Ok => virt.ok = pressed,
            Gamepad::No => virt.no = pressed,
        }
    }

    return;
}

pub(crate) fn despawn_main_menu_ui(
    mut commands: Commands,
    entity: Single<Entity, With<MainMenuUi>>,
) {
    commands.entity(*entity).despawn();
}
