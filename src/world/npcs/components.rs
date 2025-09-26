use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LdtkEntity};

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
    pub npcs_locations: HashMap<u32, GridCoords>,
}

impl LevelNPCs {
    pub fn in_npc(&self, grid_coords: &GridCoords) -> bool {
        let v = self.npcs_locations.values();
        v.collect::<Vec<_>>().contains(&grid_coords)
    }
}

pub enum NPCKind {
    Librarian,
    Monk,
    Writer,
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
