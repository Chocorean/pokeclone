use bevy::{platform::collections::HashSet, prelude::*};
use bevy_ecs_ldtk::{
    GridCoords, LdtkProjectHandle, LevelEvent,
    assets::{LdtkProject, LevelMetadataAccessor},
};

use crate::world::{GridSize, LevelWalls, walls::components::Wall};

/// Store the coordinates of a level's walls.
///
/// Only run once per level load, on [LevelEvent].
pub fn cache_wall_locations(
    mut level_walls: ResMut<LevelWalls>,
    mut level_events: EventReader<LevelEvent>,
    grid_size: Res<GridSize>,
    obstacles: Query<&GridCoords, With<Wall>>,
    ldtk_project_entities: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for level_event in level_events.read() {
        if let LevelEvent::Transformed(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single().unwrap())
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            let wall_locations: HashSet<GridCoords> = obstacles.iter().copied().collect();
            let new_level_walls = LevelWalls {
                wall_locations,
                level_width: level.px_wid / grid_size.0,
                level_height: level.px_hei / grid_size.0,
            };

            *level_walls = new_level_walls;
        }
    }
}
