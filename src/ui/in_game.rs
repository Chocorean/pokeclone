use crate::{AppState, camera::WorldTexture, event::NewSaveEvent};
use bevy::prelude::*;

#[derive(Component)]
pub struct GameUi;

pub(crate) fn setup_game_ui(mut commands: Commands, world_texture: Res<WorldTexture>) {
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
                    ..default()
                },
                BorderColor(border_color),
                BackgroundColor(bevy::color::palettes::css::RED.into()),
            ));

            // team on the side
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(800.),
                    width: Val::Px(200.),
                    height: Val::Px(700.),
                    border: UiRect::left(Val::Px(2.)),
                    ..default()
                },
                BorderColor(border_color),
                BackgroundColor(bevy::color::palettes::css::GOLD.into()),
            ));

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
