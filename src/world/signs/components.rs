use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LdtkEntity};

#[derive(Default, Component, Debug)]
pub struct Sign;

#[derive(Default, Bundle, LdtkEntity)]
pub struct SignBundle {
    sign: Sign,
    #[grid_coords]
    grid_coords: GridCoords,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}
