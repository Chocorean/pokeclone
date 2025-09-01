use bevy::prelude::*;
use bevy_ecs_ldtk::{GridCoords, LevelSelection};

use crate::{player::Player, save::Save, team::Team};

// const systems: &[] = [new_save];

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<NewSaveEvent>()
            .add_event::<LoadSaveEvent>()
            .add_systems(Update, new_save);
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
