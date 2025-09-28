use bevy::{platform::collections::HashSet, prelude::*};
use bevy_ecs_ldtk::{GridCoords, LevelEvent};

use crate::world::herbs::{LevelHerbs, components::Herb};

pub fn cache_herb_locations(
    mut level_walls: ResMut<LevelHerbs>,
    herbs: Query<&GridCoords, With<Herb>>,
    mut ev_levels: EventReader<LevelEvent>,
) {
    for ev in ev_levels.read() {
        if matches!(ev, LevelEvent::Spawned(_)) {
            // We only load one level at a time so we can exit
            return;
        }

        level_walls.herb_locations = HashSet::new(); // reset
        for herb_coords in herbs.iter() {
            level_walls.herb_locations.insert(*herb_coords);
        }
    }
}
