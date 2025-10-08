use bevy::{platform::collections::HashSet, prelude::*};
use bevy_ecs_ldtk::{GridCoords, LevelEvent};

use crate::{
    event::MoveInBushEvent,
    player::Player,
    utils::SmoothMove,
    world::herbs::{LevelHerbs, components::Herb},
};

pub fn cache_herb_locations(
    mut level_walls: ResMut<LevelHerbs>,
    herbs: Query<&GridCoords, With<Herb>>,
    mut ev_levels: EventReader<LevelEvent>,
) {
    for ev in ev_levels.read() {
        if let LevelEvent::Transformed(_) = ev {
            level_walls.herb_locations = HashSet::new(); // reset
            for herb_coords in herbs.iter() {
                level_walls.herb_locations.insert(*herb_coords);
            }
        }
    }
}

pub fn walk_in_herbs(
    gc: Single<(&GridCoords, Option<&SmoothMove>), (Changed<GridCoords>, With<Player>)>,
    level_herbs: Res<LevelHerbs>,
    mut writer: EventWriter<MoveInBushEvent>,
) {
    if level_herbs.herb_locations.contains(gc.0) && gc.1.is_none() {
        writer.write(MoveInBushEvent);
    }
}
