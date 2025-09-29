use crate::{AppState, camera::WorldTexture, event::NewSaveEvent};
use bevy::prelude::*;

#[derive(Component)]
pub struct GameUi;

pub(crate) fn setup_game_ui(mut commands: Commands, world_texture: Res<WorldTexture>) {
    commands
        .spawn((
            GameUi,
            Node {
                border: UiRect::all(Val::Px(1.)),
                width: Val::Px(800.),
                height: Val::Px(600.),
                ..default()
            },
            BorderColor(Color::linear_rgb(0.8, 0.8, 0.8)),
        ))
        .with_children(|root| {
            root.spawn(ImageNode::new(world_texture.0.clone()));
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
