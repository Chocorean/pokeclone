use bevy::prelude::*;

use crate::team::Team;
use crate::ui::game::WorldUI;

use crate::creature::Creature;

#[derive(Component)]
pub struct FightUi;

pub fn fight_ui(
    mut commands: Commands,
    ui_q: Query<Entity, With<WorldUI>>,
    creature: Res<Creature>,
    team: Res<Team>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(ui) = ui_q.single() {
        commands.entity(ui).with_children(|parent| {
            parent
                .spawn((
                    FightUi,
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                ))
                .with_children(|root| {
                    root.spawn((
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(50.),
                            flex_direction: FlexDirection::RowReverse,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.9411765, 0.99215686, 0.95686275)),
                    ))
                    .with_children(|top_row| {
                        top_row.spawn((
                            Text::new(creature.name.clone()),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ));
                        let handle = asset_server.load(creature.texture_path());
                        top_row.spawn((
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
                    });
                    // root.spawn();
                });
        });
    }
}
