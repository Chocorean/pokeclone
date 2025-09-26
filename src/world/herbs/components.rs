use bevy::{platform::collections::HashSet, prelude::*};
use bevy_ecs_ldtk::{GridCoords, LdtkIntCell};

#[derive(Default, Component)]
pub struct Herb;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct HerbBundle {
    herb: Herb,
}

#[derive(Default, Resource)]
/// Store herbs locations for event trigger.
pub struct LevelHerbs {
    pub herb_locations: HashSet<GridCoords>,
}
