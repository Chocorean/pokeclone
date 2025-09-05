use bevy::prelude::*;
use bevy_ecs_ldtk::{GridCoords, LevelSelection};

use crate::{
    AppState,
    creature::{Creature, Dex},
    player::Player,
    save::Save,
    team::Team,
};

// `Save`-related

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<NewSaveEvent>()
            .add_event::<LoadSaveEvent>()
            .add_event::<MoveInBushEvent>()
            .add_event::<WildEncounterEvent>()
            .add_systems(
                Update,
                (new_save, spawn_wild_encounter, wild_encounter).run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Event)]
/// Trigger when the player tries to save.
pub struct NewSaveEvent;

/// Gather what matters and save it all.
/// Might be a better way to do so, is that Events ? <-- TODO investigate
pub fn new_save(
    mut events: EventReader<NewSaveEvent>,
    player_q: Query<&GridCoords, With<Player>>,
    level_res: Res<LevelSelection>,
    team: Res<Team>,
) {
    for _ in events.read() {
        let level_id = match *level_res {
            LevelSelection::Indices(x) => x.level,
            _ => todo!("not supported"),
        };
        let coords = player_q.single().unwrap();
        Save::new(level_id as i32, *coords, team.clone());
    }
}

#[derive(Event)]
/// Trigger when the player loads the save.
pub struct LoadSaveEvent;

// "Wild encounter"-related events

#[derive(Event)]
/// Trigger each time the player changes direction or moves into a bush.
pub struct MoveInBushEvent;

/// Roll a dice a sent a `WildEncounterEvent` on sucess.
fn spawn_wild_encounter(
    mut move_in_bush_reader: EventReader<MoveInBushEvent>,
    mut wild_encounter_writer: EventWriter<WildEncounterEvent>,
    dex: Res<Dex>,
) {
    for _ in move_in_bush_reader.read() {
        let mut rng = rand::rng();
        let nbr = rand::Rng::random::<u8>(&mut rng);
        if nbr < 64 {
            // start a random encounter
            let creature = dex.random();
            wild_encounter_writer.write(WildEncounterEvent(creature));
        }
    }
}

#[derive(Event)]
/// Trigger before a fight against a wild foe.
pub struct WildEncounterEvent(pub Creature);

/// Prepare the data for a fight against a wild foe.
fn wild_encounter(
    mut wild_encounter_reader: EventReader<WildEncounterEvent>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in wild_encounter_reader.read() {
        commands.insert_resource(event.0.clone());
        next_state.set(AppState::InFight);
    }
}
