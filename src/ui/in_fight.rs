use bevy::prelude::*;

use crate::AppState;

#[derive(Component)]
pub struct FightUi;

pub(crate) fn setup_fight_ui(mut commands: Commands) {
    commands.spawn((
        FightUi,
        Node {
            border: UiRect::all(Val::Px(1.)),
            width: Val::Px(600.),
            height: Val::Px(400.),
            left: Val::Px(100.),
            top: Val::Px(100.),
            ..default()
        },
        BackgroundColor(Color::linear_rgba(0.2, 0.2, 0.2, 0.5)),
    ));
}

pub(crate) fn despawn_fight_ui(mut commands: Commands, entity: Single<Entity, With<FightUi>>) {
    commands.entity(*entity).despawn();
}

/// temporary, just to be able to escape the state
pub(crate) fn handle_fight_input(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if input.just_released(KeyCode::Escape) {
        next_state.set(AppState::InGame);
    }
}
