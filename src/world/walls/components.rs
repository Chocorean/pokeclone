use bevy::{platform::collections::HashSet, prelude::*};
use bevy_ecs_ldtk::{GridCoords, LdtkIntCell};

#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Default, Resource)]
/// Store walls and NPCs locations for collision checking.
pub struct LevelWalls {
    pub wall_locations: HashSet<GridCoords>,
    pub level_width: i32,
    pub level_height: i32,
}

impl LevelWalls {
    pub fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
}
