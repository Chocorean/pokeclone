use bevy::prelude::*;

use crate::{AppState, ui::VirtualInput};

#[derive(Component)]
pub(crate) struct IndexUi;

pub fn setup_index_ui(mut commands: Commands) {
    commands.spawn((
        IndexUi,
        (
            Node {
                top: Val::Percent(5.),
                left: Val::Percent(5.),
                width: Val::Percent(90.),
                height: Val::Percent(90.),
                ..default()
            },
            BackgroundColor(Color::srgba_u8(0, 0, 0, 185)),
        ),
    ));
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
    }
}
