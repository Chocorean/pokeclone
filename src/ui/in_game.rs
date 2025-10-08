use crate::{
    AppState,
    camera::WorldTexture,
    dex::Dex,
    event::{LogEvent, NewSaveEvent},
    team::Team,
    ui::widgets::{button, team_member_widget},
};
use bevy::{
    color::palettes::css::{BROWN, GREEN},
    prelude::*,
};

#[derive(Component)]
pub struct LogsUi;

#[derive(Component)]
pub struct GameUi;

#[derive(Component)]
pub struct IndexButton;

#[derive(Component)]
pub struct SaveButton;

pub fn setup_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    world_texture: Res<WorldTexture>,
    team: Res<Team>,
    dex: Res<Dex>,
) {
    let font = asset_server.load("fonts/mmc.otf");
    commands
        .spawn((
            GameUi,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                left: Val::Px(0.),
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
        ))
        .with_children(|root| {
            let border_color = Color::linear_rgb(0.8, 0.8, 1.);

            // top menu
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    width: Val::Px(800.),
                    height: Val::Px(100.),
                    border: UiRect::bottom(Val::Px(2.)),
                    padding: UiRect::all(Val::Px(10.)),
                    column_gap: Val::Px(5.),
                    ..default()
                },
                BorderColor(border_color),
                BackgroundColor(Color::linear_rgb(0.32156863, 0.36078432, 0.51568628)),
            ))
            .with_children(|top| {
                top.spawn((IndexButton, button("Index", font.clone())));
                top.spawn((SaveButton, button("Save", font.clone())));
            });

            // top logs
            root.spawn((
                LogsUi,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(400.),
                    width: Val::Px(400. - 4.),
                    height: Val::Px(100. - 4.),
                    margin: UiRect::all(Val::Px(2.)),
                    padding: UiRect::all(Val::Px(4.)),
                    border: UiRect::all(Val::Px(2.)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::FlexStart,
                    overflow: Overflow::clip(), // hide pushed-out lines
                    row_gap: Val::Px(2.),

                    ..default()
                },
                BorderRadius::all(Val::Px(5.)),
                BorderColor(BROWN.into()),
                BackgroundColor(GREEN.into()),
            ))
            // .with_children(|logs| {
            //     logs.spawn((
            //         Text::new("[time] test 1".to_string()),
            //         TextFont {
            //             font: font.clone(),
            //             font_size: 10.,
            //             ..default()
            //         },
            //     ));
            //     logs.spawn((
            //         Text::new("[time] test 2".to_string()),
            //         TextFont {
            //             font: font.clone(),
            //             font_size: 10.,
            //             ..default()
            //         },
            //     ));
            //     logs.spawn((
            //         Text::new("[time] test 3".to_string()),
            //         TextFont {
            //             font: font.clone(),
            //             font_size: 10.,
            //             ..default()
            //         },
            //     ));
            //     logs.spawn((
            //         Text::new("[time] test 4".to_string()),
            //         TextFont {
            //             font: font.clone(),
            //             font_size: 10.,
            //             ..default()
            //         },
            //     ));
            //     logs.spawn((
            //         Text::new("[time] test 5".to_string()),
            //         TextFont {
            //             font: font.clone(),
            //             font_size: 10.,
            //             ..default()
            //         },
            //     ));
            // })
            ;

            // team on the side
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(800.),
                    width: Val::Px(200.),
                    height: Val::Px(700.),
                    border: UiRect::left(Val::Px(2.)),
                    padding: UiRect::top(Val::Px(5.)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Start,
                    // row_gap: Val::Px(12.0),
                    ..default()
                },
                BorderColor(border_color),
                BackgroundColor(bevy::color::palettes::tailwind::GRAY_600.into()),
            ))
            .with_children(|team_ui| {
                for (i, member) in team.0.iter().enumerate() {
                    team_ui.spawn(team_member_widget(member.clone(), i, font.clone(), &dex));
                }
            });

            // game cam
            root.spawn((Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                top: Val::Px(100.),
                width: Val::Px(800.),
                height: Val::Px(600.),
                ..default()
            },))
                .with_children(|root| {
                    root.spawn(ImageNode::new(world_texture.0.clone()));
                });
        });
}

pub(crate) fn handle_game_ui_input(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<NewSaveEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    } else if keyboard_input.just_pressed(KeyCode::F1) {
        event_writer.write(NewSaveEvent);
    }
}

pub(crate) fn despawn_game_ui(mut commands: Commands, entity: Single<Entity, With<GameUi>>) {
    commands.entity(*entity).despawn();
}

pub fn handle_index_button(
    mut next_state: ResMut<NextState<AppState>>,
    button: Single<&Interaction, (Changed<Interaction>, With<IndexButton>)>,
) {
    if matches!(*button, Interaction::Pressed) {
        next_state.set(AppState::Index);
    }
}

pub fn handle_save_button(
    mut event_writer: EventWriter<NewSaveEvent>,
    button: Single<&Interaction, (Changed<Interaction>, With<SaveButton>)>,
) {
    if matches!(*button, Interaction::Pressed) {
        event_writer.write(NewSaveEvent);
    }
}

pub fn handle_new_log(
    mut commands: Commands,
    ui: Single<Entity, With<LogsUi>>,
    mut reader: EventReader<LogEvent>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/mmc.otf");
    for LogEvent(msg) in reader.read() {
        let child = commands
            .spawn((
                Text::new(msg),
                TextFont {
                    font: font.clone(),
                    font_size: 10.,
                    ..default()
                },
            ))
            .id();
        commands.entity(*ui).add_child(child);
    }
}
