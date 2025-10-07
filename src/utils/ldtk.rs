//! full stupid module, unwrapping like crazy
//!
//! just short functions to read ldtks entities custom data

use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, GridCoords, ldtk::FieldValue};

use crate::{utils::Direction, world::NPCKind};

/// Extract string
pub fn read_str_from_ldtk_entity(key: &str, entity: &EntityInstance) -> String {
    let err_msg = format!("key `{key}` empty/not found");
    let value = entity
        .field_instances
        .iter()
        .find(|f| f.identifier == key)
        .unwrap()
        .value
        .clone();
    match value {
        FieldValue::String(s) => s.unwrap_or(err_msg),
        _ => err_msg,
    }
}

fn read_enum_from_ldtk_entity(key: &str, entity: &EntityInstance) -> FieldValue {
    entity
        .field_instances
        .iter()
        .find(|f| f.identifier == key)
        .unwrap()
        .value
        .clone()
}

pub fn read_npc_kind_from_ldtk_entity(entity: &EntityInstance) -> NPCKind {
    match read_enum_from_ldtk_entity("kind", entity) {
        // kind is mandatory for npcs so we can unwrap
        FieldValue::Enum(s) => s.unwrap().into(),
        x => panic!(" {x:?} is not a kind ?"),
    }
}

pub fn read_dir_from_ldtk_entity(entity: &EntityInstance) -> Direction {
    match read_enum_from_ldtk_entity("direction", entity) {
        FieldValue::Enum(s) => s.unwrap().into(),
        x => panic!(" {x:?} is not a direction ?"),
    }
}

// The change of origin (top-left for ldtk vs bottom-left for bevy) should be done in a better way but I'm burnt out.
pub fn read_direction_from_ldtk_entity(entity: &EntityInstance) -> Option<Vec<GridCoords>> {
    let value = entity
        .field_instances
        .iter()
        .find(|f| f.identifier == "destinations")?
        .value
        .clone();
    match value {
        FieldValue::Points(vec) => Some(
            vec.iter()
                .map(|x| {
                    let iv = x.expect(&format!(
                        "ldtk destinations malformed for entity {}",
                        entity.iid
                    ));
                    GridCoords {
                        x: iv.x,
                        y: 24 - iv.y,
                    }
                })
                .collect(),
        ),
        _ => None,
    }
}
