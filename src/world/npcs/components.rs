use std::fmt;

use bevy::{platform::collections::HashSet, prelude::*};
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LdtkEntity};

use crate::utils::GC;

// npc trainer uuid
// ca7c1690-5e50-11f0-85ca-e96bd84a6222

#[derive(Default, Component, Debug, Hash, PartialEq, Eq, Clone)]
pub struct NPC;

#[derive(Default, Bundle, LdtkEntity)]
pub struct NPCsBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
    npc: NPC,
    #[grid_coords]
    grid_coords: GridCoords,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Default, Resource)]
/// Store walls and NPCs locations for collision checking.
pub struct LevelNPCs {
    /// the key comes from `entity.index()`
    pub npcs_locations: HashSet<GridCoords>,
}

impl LevelNPCs {
    pub fn in_npc(&self, grid_coords: &GridCoords) -> bool {
        self.npcs_locations.contains(grid_coords)
    }
}

#[derive(Debug)]
pub enum NPCKind {
    Librarian,
    Monk,
    Writer,
}
impl fmt::Display for NPCKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<String> for NPCKind {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Librarian" => Self::Librarian,
            "Monk" => Self::Monk,
            "Writer" => Self::Writer,
            _ => panic!("unkown npc kind"),
        }
    }
}

/// This component carries three pieces of data:
/// path: [Vec]<[GridCoords]> where we store the full path
/// current: [usize] the current path
/// next: [GridCoords] the grid coords the NPC is going to
#[derive(Component, Debug, Clone)]
pub(crate) struct MovingNPCSchedule {
    pub path: Vec<GridCoords>,
    pub current: usize,
    pub next: GridCoords,
}

impl MovingNPCSchedule {
    /// Return the `next` destination, and update internal state
    ///
    /// If the next destination is contained in `path`, we need to
    /// increase the `current` index
    pub fn update(&mut self) -> GridCoords {
        let next = self.next.clone();
        if self.path.contains(&next) {
            self.current = (self.current + 1) % self.path.len();
        }
        let next_path_dest = self.path[self.current];
        // find next destination and save it
        let new_next = next.next_step(next_path_dest);
        // at this point, diff should have the right direction, and be
        // a neighbor tile to `next`
        self.next = new_next;
        next
    }
}
