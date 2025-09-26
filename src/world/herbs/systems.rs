use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::world::herbs::{LevelHerbs, components::Herb};

pub fn cache_herb_locations(
    mut level_walls: ResMut<LevelHerbs>,
    herbs: Query<&GridCoords, With<Herb>>,
) {
    for herb_coords in herbs.iter() {
        level_walls.herb_locations.insert(*herb_coords);
    }
}
