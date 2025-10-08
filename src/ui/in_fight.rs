use bevy::prelude::*;
use bevy_easy_gif::GifNode;

use crate::{
    AppState,
    dex::{Creature, Dex},
    event::LogEvent,
    team::{Team, TeamMember},
    ui::widgets::button,
};

#[derive(Component)]
pub struct AttackButton;

#[derive(Component)]
pub struct TameButton;

#[derive(Component)]
pub struct FightUi;

pub(crate) fn setup_fight_ui(
    mut commands: Commands,
    foe: Res<Creature>,
    team: Res<Team>,
    dex: Res<Dex>,
    mut writer: EventWriter<LogEvent>,
    asset_server: Res<AssetServer>,
) {
    writer.write(LogEvent(format!("A wild {} appears!", foe.name)));
    let font = asset_server.load("fonts/mmc.otf");

    commands
        .spawn((
            FightUi,
            Node {
                position_type: PositionType::Absolute,
                border: UiRect::all(Val::Px(1.)),
                width: Val::Px(700.),
                height: Val::Px(500.),
                left: Val::Px(50.),
                top: Val::Px(150.),
                ..default()
            },
            BackgroundColor(Color::linear_rgba(0.2, 0.2, 0.2, 0.95)),
        ))
        .with_children(|root| {
            // GifNodes
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(50.),
                    width: Val::Percent(100.),
                    height: Val::Px(128.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                children![
                    (
                        GifNode {
                            handle: team.0.first().unwrap().handle(&dex),
                        },
                        ImageNode {
                            flip_x: true,
                            ..default()
                        }
                    ),
                    GifNode {
                        handle: dex.get_creature(dex.get_creature_ids(&foe)).1.clone()
                    }
                ],
            ));
            // Attack and Tame buttons
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(250.),
                    width: Val::Percent(100.),
                    height: Val::Px(40.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                children![
                    (AttackButton, button("Attack", font.clone())),
                    (TameButton, button("Tame", font)),
                ],
            ));
        });
}

pub(crate) fn despawn_fight_ui(mut commands: Commands, entity: Single<Entity, With<FightUi>>) {
    commands.entity(*entity).despawn();
    commands.remove_resource::<Creature>(); // clean last foe
}

pub(crate) fn handle_attack_button(
    mut commands: Commands,
    mut writer: EventWriter<LogEvent>,
    button: Single<&Interaction, (Changed<Interaction>, With<AttackButton>)>,
) {
    if !matches!(*button, Interaction::Pressed) {
        return;
    }

    // First element of sequence here so it doesn't wait, then start it
    writer.write(LogEvent(
        "[FIGHT] A thunderbolt comes from the highest skies and lands right on your foe..."
            .to_string(),
    ));
    commands.spawn(AttackSequence {
        stage: 0,
        timer: Timer::from_seconds(1.4, TimerMode::Once),
    });
}

#[derive(Component)]
pub struct AttackSequence {
    stage: u8,
    timer: Timer,
}

/// Print a few things, wait betweem, and goes back to InGame state
pub(crate) fn progress_attack_sequence(
    mut commands: Commands,
    time: Res<Time>,
    q: Single<(Entity, &mut AttackSequence)>,
    mut writer: EventWriter<LogEvent>,
    foe: Res<Creature>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let (e, mut seq) = q.into_inner();
    seq.timer.tick(time.delta());
    if !seq.timer.finished() {
        return;
    }

    match seq.stage {
        0 => {
            writer.write(LogEvent(format!(
                "[FIGHT] Enemy {} gets annihilated!!",
                foe.name
            )));
            seq.stage = 1;
            seq.timer = Timer::from_seconds(1., TimerMode::Once);
        }
        _ => {
            next_state.set(AppState::InGame);
            commands.entity(e).despawn();
        }
    }
}

pub(crate) fn handle_tame_button(
    mut commands: Commands,
    mut writer: EventWriter<LogEvent>,
    foe: Res<Creature>,
    button: Single<&Interaction, (Changed<Interaction>, With<TameButton>)>,
) {
    if !matches!(*button, Interaction::Pressed) {
        return;
    }

    // First element of sequence here so it doesn't wait, then start it
    writer.write(LogEvent(format!(
        "[FIGHT] You seem to bound quite easily with the wild {}...",
        foe.name
    )));
    commands.spawn(TameSequence {
        stage: 0,
        timer: Timer::from_seconds(1.4, TimerMode::Once),
    });
}

#[derive(Component)]
pub struct TameSequence {
    stage: u8,
    timer: Timer,
}

pub(crate) fn progress_tame_sequence(
    mut commands: Commands,
    time: Res<Time>,
    q: Single<(Entity, &mut TameSequence)>,
    mut writer: EventWriter<LogEvent>,
    foe: Res<Creature>,
    mut team: ResMut<Team>,
    dex: Res<Dex>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let (e, mut seq) = q.into_inner();
    seq.timer.tick(time.delta());
    if !seq.timer.finished() {
        return;
    }

    match seq.stage {
        0 => {
            writer.write(LogEvent(format!(
                "[FIGHT] Enemy {} joins your team.",
                foe.name
            )));
            // add to team
            let new_member = TeamMember::from(&foe, &dex);
            if team.0.len() == 5 {
                team.0.remove(0);
            }
            team.0.push(new_member);
            // go back to game
            next_state.set(AppState::InGame);
            commands.entity(e).despawn();
            // HERE
        }
        _ => {
            next_state.set(AppState::InGame);
            commands.entity(e).despawn();
        }
    }
}
