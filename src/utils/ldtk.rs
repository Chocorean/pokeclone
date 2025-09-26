//! full stupid module, unwrapping like crazy
//!
//! just short functions to read ldtks entities custom data

use bevy_ecs_ldtk::{EntityInstance, ldtk::FieldValue};

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
