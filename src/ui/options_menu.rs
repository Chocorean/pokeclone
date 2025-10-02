use bevy::prelude::*;

use crate::{AppState, ui::widgets::*};

#[derive(Component)]
pub struct ReturnButton;

pub fn handle_return_button(
    button: Single<&Interaction, (Changed<Interaction>, With<ReturnButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if matches!(*button, Interaction::Pressed) {
        next_state.set(AppState::MainMenu);
    }
}

#[derive(Component)]
pub struct OptionsUi;

pub(crate) fn setup_options_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/mmc.otf");
    commands
        .spawn((
            OptionsUi,
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
            .with_children(|parent| {
                parent.spawn((
                    Text("Options".to_string()),
                    TextFont {
                        font: font.clone(),
                        ..default()
                    },
                ));
                parent.spawn(button("Return", font)).insert(ReturnButton);
            });
        });
}

pub(crate) fn despawn_options_ui(
    mut commands: Commands,
    entity: Single<Entity, With<ReturnButton>>,
) {
    commands.entity(*entity).despawn();
}
