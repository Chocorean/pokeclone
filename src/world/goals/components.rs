use bevy::{platform::collections::HashSet, prelude::*};
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LdtkEntity};

use crate::utils::Direction;

// level 0 Goal
// f634c0c0-5e50-11f0-a81f-7d5d71ee8bd5
// level 1 goal
// 99a501a0-8560-11f0-ab4e-67d06badcd69

#[derive(Default, Component)]
pub(crate) struct Goal;

#[derive(Default, Bundle, LdtkEntity)]
pub struct GoalBundle {
    goal: Goal,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Default, Resource)]
pub struct LevelGoals {
    pub goal_locations: HashSet<GridCoords>,
}

/// Insert in an Entity to be teleported.
///
/// The [String] refers to the destination [Goal] entity_iid.
///
/// The [Direction] will determine the relative landing position
/// (position of the [Goal] entity + 1 in the given direction)
#[derive(Component)]
pub struct WaitingTeleport(pub Direction, pub String);
